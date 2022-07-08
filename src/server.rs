//! Core server code is included in this module.

use std::{io, sync::Arc};

use tokio::io::Interest;

pub struct Server {
	max_connections: usize,
	listener: tokio::net::TcpListener,

	pub(crate) shutdown_tx: tokio::sync::mpsc::Sender<()>,
	pub(crate) shutdown_rx: tokio::sync::mpsc::Receiver<()>,
	pub(crate) shutdown_notifier: tokio::sync::broadcast::Sender<()>,
}

impl Server {
	#[inline(always)]
	pub(crate) async fn new(
		address: String,
		max_connections: usize,
		shutdown_notifier: tokio::sync::broadcast::Sender<()>,
	) -> io::Result<Self> {
		let listener = tokio::net::TcpListener::bind(address).await?;
		let (shutdown_tx, shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);
		Ok(Self {
			listener,
			max_connections,

			shutdown_rx,
			shutdown_tx,
			shutdown_notifier,
		})
	}

	pub(crate) async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
		// Limits the number of clients concurrently connected to the instance.
		let semaphore = Arc::new(tokio::sync::Semaphore::new(self.max_connections));

		loop {
			if let Ok((mut stream, addr)) = self.listener.accept().await {
				let semaphore = Arc::clone(&semaphore);
				tokio::spawn(async move {
					if let Ok(guard) = semaphore.try_acquire() {
						if let Ok(state) =
							stream.ready(Interest::READABLE | Interest::WRITABLE).await
						{}

						drop(guard);
					}
				});
			} else {
				return Err("tcp/ip listener cannot accept inbound connection.".into());
			}
		}

		Ok(())
	}
}
