use std::net::SocketAddr;

use crate::Unspecified;

pub struct Server(&'static str);

impl Default for Server {
    /// Creates a server at `127.0.0.1:0`, assigning it to a random port.
    fn default() -> Self {
        Self("127.0.0.1:0")
    }
}

impl Server {
    pub async fn run(&self, cb: Option<Box<dyn FnOnce(SocketAddr) -> ()>>) -> Unspecified<()> {
        let listener = tokio::net::TcpListener::bind(&self.0).await?;
        let address = listener.local_addr()?;
        cb.unwrap_or(Box::new(|_| {}))(address);

        loop {
            if let Ok((stream, _)) = listener.accept().await {
                if let Err(error) = stream.writable().await {
                    eprint!("error: {}", error);
                }

                loop {
                    unimplemented!()
                }
            }
        }
    }
}
