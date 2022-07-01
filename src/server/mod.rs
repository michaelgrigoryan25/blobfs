use std::{io::ErrorKind, net::SocketAddr};

use crate::Unspecified;

/// TCP server, built on top of [tokio::net::TcpListener].
pub struct Server {
    address: String,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            address: String::from("127.0.0.1:0"),
        }
    }
}

impl Server {
    /// Creates a new server. If the provided address is [None], this will default
    /// to [Server::default].
    pub fn new(address: Option<String>) -> Self {
        match address {
            Some(address) => Self { address },
            None => Self::default(),
        }
    }

    /// Starts the server, and executes optionally provided callback allocated on
    /// the heap. This function blocks indefinitely, and returns an error, once a
    /// critical error occurs.
    pub async fn run(&self, cb: Option<Box<dyn FnOnce(SocketAddr)>>) -> Unspecified<()> {
        let listener = tokio::net::TcpListener::bind(&self.address).await?;
        let address: SocketAddr = listener.local_addr()?;
        // Using unwrap_or_else, since unwrap_or will potentially allocate a separate
        // object for the current object.
        cb.unwrap_or_else(|| Box::new(|addr| info!("tcp server started at: {}", addr)))(address);
        drop(address); // we can now consider dropping the address variable, since it is unused

        loop {
            if let Ok((stream, _)) = listener.accept().await {
                if stream.readable().await.is_ok() {
                    // Creating the buffer after the `await` prevents it from being
                    // stored in the async task.
                    let mut buffer: Vec<u8> = vec![0; 1024];
                    match stream.try_read(&mut buffer) {
                        Ok(count) => debug!("read {} bytes", count),
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => continue,
                        Err(e) => error!("error while reading the stream: {}", e),
                    }
                }

                if stream.writable().await.is_ok() {
                    match stream.try_write(b"test") {
                        Ok(_) => continue,
                        Err(_) => continue,
                    }
                }
            }
        }
    }
}
