#[macro_use]
extern crate axum_debug;

use crate::{
    config::{Config, ConfigSingletonReader},
    handlers::{file, remove, upload},
    util::addr,
};
use axum::{
    routing::{any, delete, get, post},
    Router,
};
use hyper::StatusCode;
use std::net::SocketAddr;

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

    // Reading the configuration through the `ConfigSingletonReader`
    let config = ConfigSingletonReader::singleton()
        .inner
        .lock()
        .expect("Thread was not able to lock `ConfigSingletonReader`");

    let port: u16 = addr::get_port(&config);
    let addr: SocketAddr = addr::get_addr(&config, port);

    // Dropping the configuration to make it available
    // to all threads, otherwise it will stay locked
    // in current scope and we won't be able to use
    // the singleton anymore.
    drop(config);

    // Initializing the routes
    let stormi = Router::new()
        // To not get random errors from `file::handler`
        .route("/favicon.ico", any(|| async { StatusCode::NOT_FOUND }))
        .route("/:hash", get(file::handler))
        .route("/upload", post(upload::handler))
        .route("/remove", delete(remove::handler));

    let server = axum::Server::bind(&addr).serve(stormi.into_make_service());

    println!("> Stormi started at {}", addr);
    if let Err(error) = server.await {
        eprintln!("{}", &error);
    }
}
