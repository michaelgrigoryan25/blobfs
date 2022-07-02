use std::io::{self, ErrorKind};

/// TCP server implementation built on top of [tokio::net::TcpListener].
pub struct Server {
	pub(crate) listener: tokio::net::TcpListener,

	/// The channel that is going to be used for sending a shutdown signal to
	/// the caller for graceful shutdown handling.
	#[allow(unused)]
	shutdown: tokio::sync::broadcast::Sender<()>,
}

impl Server {
	pub(crate) async fn new(
		address: Option<String>,
		shutdown: tokio::sync::broadcast::Sender<()>,
	) -> io::Result<Self> {
		let address = address.unwrap_or_else(|| "127.0.0.1:0".into());
		let listener = tokio::net::TcpListener::bind(address).await?;
		Ok(Self { listener, shutdown })
	}

	/// Starts the server, and executes optionally provided callback allocated on
	/// the heap. This function blocks indefinitely, and returns an error, once a
	/// critical error occurs.
	pub(crate) async fn run(self) -> io::Result<()> {
		loop {
			if let Ok((stream, ref addr)) = self.listener.accept().await {
				trace!("tcp/ip connection established: {}", addr);
				if stream.readable().await.is_ok() {
					// Creating the buffer after the `await` prevents it from being
					// stored in the async task.
					let mut buffer = [0; 512];
					match stream.try_read(&mut buffer) {
						Ok(read) => info!("{}", String::from_utf8_lossy(&buffer[..read])),
						Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
						Err(e) => error!("error while reading the stream: {}", e),
					}
				}

				trace!("tcp/ip connection closed: {}", addr);
			}
		}
	}
}
