//! Core server code is included in this module.

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use bytes::BytesMut;
use tokio::io::BufWriter;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, Semaphore};

use crate::cmd;
use crate::shutdown::Shutdown;

/// Server listener state. Created in the `run` call. It includes a `run` method
/// which performs the TCP listening and initialization of per-connection state.
struct Server {
	listener: TcpListener,

	/// Limit the max number of connections.
	///
	/// A [Semaphore] is used to limit the max number of connections. Before
	/// attempting to accept a new connection, a permit is acquired from the
	/// semaphore. If none are available, the listener waits for one.
	///
	/// When handlers complete processing a connection, the permit is returned
	/// to the semaphore.
	limiter: Arc<Semaphore>,

	/// Broadcasts a shutdown signal to all active connections.
	///
	/// The initial `shutdown` trigger is provided by the `run` caller. The
	/// server is responsible for gracefully shutting down active connections.
	/// When a connection task is spawned, it is passed a broadcast receiver
	/// handle. When a graceful shutdown is initiated, a `()` value is sent via
	/// the broadcast::Sender. Each active connection receives it, reaches a
	/// safe terminal state, and completes the task.
	notify_shutdown: broadcast::Sender<()>,

	/// Used as part of the graceful shutdown process to wait for client
	/// connections to complete processing.
	///
	/// Tokio channels are closed once all `Sender` handles go out of scope.
	/// When a channel is closed, the receiver receives `None`. This is
	/// leveraged to detect all connection handlers completing. When a
	/// connection handler is initialized, it is assigned a clone of
	/// `shutdown_complete_tx`. When the listener shuts down, it drops the
	/// sender held by this `shutdown_complete_tx` field. Once all handler tasks
	/// complete, all clones of the `Sender` are also dropped. This results in
	/// `shutdown_complete_rx.recv()` completing with `None`. At this point, it
	/// is safe to exit the server process.
	shutdown_complete_tx: mpsc::Sender<()>,
	shutdown_complete_rx: mpsc::Receiver<()>,
}

impl Server {
	/// Run the server
	///
	/// Listen for inbound connections. For each inbound connection, spawn a
	/// task to process that connection.
	///
	/// # Errors
	///
	/// Returns `Err` if accepting returns an error. This can happen for a
	/// number reasons that resolve over time. For example, if the underlying
	/// operating system has reached an internal limit for max number of
	/// sockets, accept will fail.
	///
	/// The process is not able to detect when a transient error resolves
	/// itself. One strategy for handling this is to implement a back off
	/// strategy, which is what we do here.
	async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		loop {
			// Wait for a permit to become available
			//
			// `acquire_owned` returns a permit that is bound to the semaphore.
			// When the permit value is dropped, it is automatically returned
			// to the semaphore.
			//
			// `acquire_owned()` returns `Err` when the semaphore has been
			// closed. We don't ever close the sempahore, so `unwrap()` is safe.
			let permit = self.limiter.clone().acquire_owned().await.unwrap();
			// Accept a new stream. This will attempt to perform error handling.
			// The `accept` method internally attempts to recover errors, so an
			// error here is non-recoverable.
			let stream = self.accept().await?;
			// Create the necessary per-connection handler state.
			let mut handler = Handler {
				connection: Connection::new(stream),
				shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
			};

			// Spawn a new task to process the connections. Tokio tasks are like
			// asynchronous green threads and are executed concurrently.
			tokio::spawn(async move {
				// Process the connection. If an error is encountered, log it.
				if let Err(err) = handler.run() {
					error!("tcp/ip handler error: {}", err);
				}

				// Move the permit into the task and drop it after completion.
				// This returns the permit back to the semaphore.
				drop(permit);
			});
		}
	}

	/// Accept an inbound connection.
	///
	/// Errors are handled by backing off and retrying. An exponential backoff
	/// strategy is used. After the first failure, the task waits for 1 second.
	/// After the second failure, the task waits for 2 seconds. Each subsequent
	/// failure doubles the wait time. If accepting fails on the 6th try after
	/// waiting for 64 seconds, then this function returns with an error.
	async fn accept(&mut self) -> Result<TcpStream, Box<dyn std::error::Error>> {
		let mut backoff = 1;
		loop {
			// Perform the accept operation. If a socket is successfully
			// accepted, return it. Otherwise, save the error.
			match self.listener.accept().await {
				Ok((socket, _)) => return Ok(socket),
				Err(err) => {
					if backoff > 64 {
						// Accept has failed too many times. Return the error.
						return Err(err.into());
					}
				}
			}

			// Pause execution until the back off period elapses.
			tokio::time::sleep(Duration::from_secs(backoff)).await;

			// Double the back off
			backoff *= 2;
		}
	}
}

/// Send and receive `Frame` values from a remote peer.
///
/// When implementing networking protocols, a message on that protocol is
/// often composed of several smaller messages known as frames. The purpose of
/// `Connection` is to read and write frames on the underlying `TcpStream`.
///
/// To read frames, the `Connection` uses an internal buffer, which is filled
/// up until there are enough bytes to create a full frame. Once this happens,
/// the `Connection` creates the frame and returns it to the caller.
///
/// When sending frames, the frame is first encoded into the write buffer.
/// The contents of the write buffer are then written to the socket.
struct Connection {
	// The `TcpStream`. It is decorated with a `BufWriter`, which provides write
	// level buffering. The `BufWriter` implementation provided by Tokio is
	// sufficient for our needs.
	stream: BufWriter<TcpStream>,

	/// The buffer for reading TCP/IP frames.
	buffer: bytes::BytesMut,
}

impl Connection {
	pub(crate) fn new(stream: TcpStream) -> Self {
		Self {
			stream: BufWriter::new(stream),
			buffer: BytesMut::with_capacity(4 * 1024 * 1024),
		}
	}
}

/// The default handler for all TCP connections. All the framing logic
/// is handled by this struct.
struct Handler {
	/// Listen for shutdown notifications.
	///
	/// A wrapper around the `broadcast::Receiver` paired with the sender in
	/// `Listener`. The connection handler processes requests from the
	/// connection until the peer disconnects **or** a shutdown notification is
	/// received from `shutdown`. In the latter case, any in-flight work being
	/// processed for the peer is continued until it reaches a safe state, at
	/// which point the connection is terminated.
	shutdown: Shutdown,

	/// When `Listener` receives an inbound connection, the `TcpStream` is
	/// passed to `Connection::new`, which initializes the associated buffers.
	/// `Connection` allows the handler to operate at the "frame" level and keep
	/// the byte level protocol parsing details encapsulated in `Connection`.
	connection: Connection,
}

impl Handler {
	pub(crate) fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		// As long as the shutdown signal has not been received, try to read a
		// new request frame.
		while !self.shutdown.is_shutdown() {}

		Ok(())
	}
}

/// Run the vxs server.
///
/// Accepts connections from the supplied listener. For each inbound connection,
/// a task is spawned to handle that connection. The server runs until the
/// `shutdown` future completes, at which point the server shuts down
/// gracefully.
///
/// [tokio::signal::ctrl_c] can be used as the `shutdown` argument. This will
/// listen for a `SIGINT` signal.
pub async fn run(listener: TcpListener, args: cmd::VxCommandStartArgs, shutdown: impl Future) {
	// When the provided `shutdown` future completes, we must send a shutdown
	// message to all active connections. We use a broadcast channel for this
	// purpose. The call below ignores the receiver of the broadcast pair, and when
	// a receiver is needed, the subscribe() method on the sender is used to create
	// one.
	let (notify_shutdown, _) = broadcast::channel::<()>(1);
	let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel::<()>(1);
	let mut server = Server {
		listener,
		notify_shutdown,
		shutdown_complete_rx,
		shutdown_complete_tx,
		limiter: Semaphore::new(args.max_connections).into(),
	};

	// Concurrently run the server and listen for the `shutdown` signal. The
	// server task runs until an error is encountered, so under normal
	// circumstances, this `select!` statement runs until the `shutdown` signal
	// is received.
	//
	// `select!` statements are written in the form of:
	//
	// ```
	// <result of async op> = <async op> => <step to perform with result>
	// ```
	//
	// All `<async op>` statements are executed concurrently. Once the **first**
	// op completes, its associated `<step to perform with result>` is
	// performed.
	//
	// The `select!` macro is a foundational building block for writing
	// asynchronous Rust. See the API docs for more details:
	//
	// https://docs.rs/tokio/*/tokio/macro.select.html
	tokio::select! {
		res = server.run() => {
			// If an error is received here, accepting connections from the TCP
			// listener failed multiple times and the server is giving up and
			// shutting down.
			//
			// Errors encountered when handling individual connections do not
			// bubble up to this point.
		}
		_ = shutdown => {
			// The shutdown signal has been received.
		}
	}

	// Extract the `shutdown_complete` receiver and transmitter
	// explicitly drop `shutdown_transmitter`. This is important, as the
	// `.await` below would otherwise never complete.
	let Server {
		mut shutdown_complete_rx,
		shutdown_complete_tx,
		notify_shutdown,
		..
	} = server;

	// When `notify_shutdown` is dropped, all tasks which have `subscribe`d will
	// receive the shutdown signal and can exit
	drop(notify_shutdown);
	// Drop final `Sender` so the `Receiver` below can complete
	drop(shutdown_complete_tx);

	// Wait for all active connections to finish processing. As the `Sender`
	// handle held by the listener has been dropped above, the only remaining
	// `Sender` instances are held by connection handler tasks. When those drop,
	// the `mpsc` channel will close and `recv()` will return `None`.
	let _ = shutdown_complete_rx.recv().await;
}
