#[macro_use]
extern crate log;
#[macro_use]
extern crate axum_debug;

use crate::{
    handlers::{not_found, remove, upload},
    util::init_loggers,
};
use axum::{
    handler::{delete, post, Handler},
    Router,
};
use std::{env, process::exit};

mod handlers;
mod util;

#[tokio::main]
async fn main() {
    // Initializing the loggers
    init_loggers().expect("Cannot initialize the loggers");

    // Getting a custom port from the environment or binding to the default one
    let port = match env::var("STORMI_PORT") {
        Ok(port) => port.parse::<u16>().unwrap_or_else(|_| {
            let message = format!(
                "`STORMI_PORT` should be a 16-bit unsigned integer. {} is out of range",
                &port
            );
            error!("{}", &message);
            exit(1)
        }),
        _ => 6345,
    };

    // Parsing the address as a `SocketAddr` to use with server
    let addr = format!("127.0.0.1:{}", &port).parse().unwrap_or_else(|_| {
        error!("Invalid `SocketAddr` was supplied");
        exit(1)
    });

    // Initializing the routes
    let stormi = Router::new()
        .route("/upload", post(upload::handler))
        .route("/remove", delete(remove::handler))
        .or(not_found::handler.into_service());

    let server = axum::Server::bind(&addr).serve(stormi.into_make_service());

    info!("Stormi started at {}", &addr);

    if let Err(error) = server.await {
        error!("{}", &error);
    }
}
