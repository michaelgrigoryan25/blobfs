use std::{
	io::{self, ErrorKind},
	sync::Arc,
};

use tokio::io::AsyncReadExt;

use crate::{FromResulting, IntoResulting};

/// TCP server implementation built on top of [tokio::net::TcpListener].
pub struct Server {
	pub(crate) listener: tokio::net::TcpListener,
	// pub(crate) shutdown_tx: tokio::sync::mpsc::Sender<()>,
	// pub(crate) shutdown_rx: tokio::sync::mpsc::Receiver<()>,
}

impl Server {
	/// Creates a new TCP server, and binds to the provided IP address.
	pub(crate) async fn new(address: String) -> io::Result<Self> {
		let listener = tokio::net::TcpListener::bind(address).await?;
		Ok(Self { listener })
	}

	/// Starts the server, and executes optionally provided callback allocated on
	/// the heap. This function blocks indefinitely, and returns an error, once a
	/// critical error occurs.
	pub(crate) async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		let semaphore = Arc::new(tokio::sync::Semaphore::new(5));
		loop {
			let semaphore = Arc::clone(&semaphore);
			let (mut stream, addr) = self.listener.accept().await?;
			tokio::spawn(async move {
				if let Ok(guard) = semaphore.try_acquire() {
					let mut buffer = [0; 1024];
					loop {
						match stream.read(&mut buffer).await {
							Ok(0) => continue, // if the stream is empty, looping until it isn't
							Ok(n) => debug!("{}", String::from_utf8_lossy(&buffer[..n])),
							Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
								ServerStatusResponse::Pending.write(&stream).await.unwrap()
							}
							Err(e) => {
								error!("error reading tcp/ip stream: {}", e);
								break;
							}
						}
					}

					// freeing one guard slot after the error occurs, so that a new
					// connection can be established.
					drop(guard);
				} else {
					error!(
						"tcp/ip connection from {} refused: too many connections",
						addr
					);
				}
			});
		}
	}
}

#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum ServerStatusResponse {
	Pending = 0x001,
	Unknown = 0x000,
}

impl ServerStatusResponse {
	pub(crate) async fn write(
		self,
		stream: &tokio::net::TcpStream,
	) -> Result<(), Box<dyn std::error::Error>> {
		stream.try_write(&self.into_resulting()?)?;
		Ok(())
	}
}

impl FromResulting<bytes::Bytes, Box<bincode::ErrorKind>> for ServerStatusResponse {
	fn from_resulting(bytes: bytes::Bytes) -> Result<Self, Box<bincode::ErrorKind>> {
		Ok(bincode::deserialize(&bytes)?)
	}
}

impl IntoResulting<bytes::Bytes, Box<bincode::ErrorKind>> for ServerStatusResponse {
	fn into_resulting(self) -> Result<bytes::Bytes, Box<bincode::ErrorKind>> {
		Ok(bincode::serialize(&self)?.into())
	}
}
