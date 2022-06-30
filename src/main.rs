use vxs::server::Server;

#[tokio::main]
async fn main() {
    Server::default().run(None).await.unwrap();
}
