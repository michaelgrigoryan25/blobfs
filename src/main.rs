#[macro_use]
extern crate axum_debug;

use crate::{
    config::Config,
    handlers::{file, remove, upload},
};
use axum::{
    routing::{any, delete, get, post},
    Router,
};
use hyper::StatusCode;
use std::{env, process::exit};

mod config;
mod handlers;
mod middleware;
mod util;

/// ASCII text that will be printed on Stormi's startup
const ASCII_BANNER: &str = r#"
 ______     ______   ______     ______     __    __     __
/\  ___\   /\__  _\ /\  __ \   /\  == \   /\ "-./  \   /\ \
\ \___  \  \/_/\ \/ \ \ \/\ \  \ \  __<   \ \ \-./\ \  \ \ \
 \/\_____\    \ \_\  \ \_____\  \ \_\ \_\  \ \_\ \ \_\  \ \_\
  \/_____/     \/_/   \/_____/   \/_/ /_/   \/_/  \/_/   \/_/
"#;

#[tokio::main]
async fn main() {
    println!("{}", ASCII_BANNER);
    println!("> Stormi is starting...");

    // Loading the configuration and logging information
    Config::init();

    // Getting a custom port from the environment or binding to the default one
    let port = match env::var("STORMI_PORT") {
        Ok(port) => port.parse::<u16>().unwrap_or_else(|_| {
            eprintln!(
                "`STORMI_PORT` should be a 16-bit unsigned integer. {} is out of range",
                port
            );
            exit(1)
        }),
        _ => 6345,
    };

    // Parsing the address as a `SocketAddr` to use with server
    let addr = format!("127.0.0.1:{}", port).parse().unwrap_or_else(|_| {
        eprintln!("Invalid `SocketAddr` was supplied");
        exit(1)
    });

    // Initializing the routes
    let stormi = Router::new()
        // To not get random errors from `file::handler`
        .route("/favicon.ico", any(|| async { StatusCode::NOT_FOUND }))
        .route("/:hash", get(file::handler))
        .route("/upload", post(upload::handler))
        .route("/remove", delete(remove::handler));
    // .route_layer();

    let server = axum::Server::bind(&addr).serve(stormi.into_make_service());

    println!("> Stormi started at {}", addr);
    if let Err(error) = server.await {
        eprintln!("{}", &error);
    }
}
