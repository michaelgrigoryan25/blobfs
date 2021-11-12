#[macro_use]
extern crate axum_debug;

use crate::handlers::{file, not_found, remove, upload};
use axum::{
    handler::Handler,
    routing::{any, delete, get, post},
    Router,
};
use hyper::StatusCode;
use std::{env, process::exit};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod handlers;
mod middleware;
mod util;

#[tokio::main]
async fn main() {
    // Getting a custom port from the environment or binding to the default one
    let port = match env::var("STORMI_PORT") {
        Ok(port) => port.parse::<u16>().unwrap_or_else(|_| {
            let message = format!(
                "`STORMI_PORT` should be a 16-bit unsigned integer. {} is out of range",
                &port
            );
            eprintln!("{}", &message);
            exit(1)
        }),
        _ => 6345,
    };

    // Parsing the address as a `SocketAddr` to use with server
    let addr = format!("127.0.0.1:{}", &port).parse().unwrap_or_else(|_| {
        eprintln!("Invalid `SocketAddr` was supplied");
        exit(1)
    });

    // Initializing the routes
    let stormi = Router::new()
        .route("/:hash", get(file::handler))
        // To not get random errors from `file::handler`
        .route("/favicon.ico", any(|| async { StatusCode::NOT_FOUND }))
        .route("/upload", post(upload::handler))
        // TODO: Add authentication layer
        .route("/remove", delete(remove::handler))
        .fallback(not_found::handler.into_service())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let server = axum::Server::bind(&addr).serve(stormi.into_make_service());

    println!("Stormi started at {}", &addr);

    if let Err(error) = server.await {
        eprintln!("{}", &error);
    }
}
