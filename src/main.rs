#[macro_use]
extern crate axum_debug;

use crate::handlers::{remove, upload};
use axum::{
    handler::{delete, post},
    Router,
};
use std::env;

mod handlers;
mod util;

#[tokio::main]
async fn main() {
    // Getting a custom port from the environment or binding to the default one
    let port = match env::var("STORMI_PORT") {
        Ok(port) => port
            .parse::<u16>()
            .expect("`STORMI_PORT` should be a 16-bit unsigned integer."),
        _ => 6345,
    };

    let addr = format!("127.0.0.1:{}", &port);

    // Initialize the routes
    let stormi = Router::new()
        .route("/upload", post(upload::handler))
        .route("/remove", delete(remove::handler));

    let server = axum::Server::bind(&addr.parse().expect("Invalid bind hostname"))
        .serve(stormi.into_make_service());

    println!("Stormi started at {}", &addr);

    if let Err(error) = server.await {
        eprintln!("{}", &error);
    }
}
