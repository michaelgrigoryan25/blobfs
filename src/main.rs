#[macro_use]
extern crate axum_debug;

use crate::{
    handlers::{file, not_found, remove, upload},
    middleware::auth::Authentication,
    util::config::{ConfigSingletonReader, ConfigTrait, DEFAULT_PERMISSIONS, DEFAULT_USERNAME},
};
use axum::{
    extract::extractor_middleware,
    handler::Handler,
    routing::{any, delete, get, post},
    Router,
};
use hyper::StatusCode;
use std::{env, process::exit};

mod handlers;
mod middleware;
mod util;

const ASCII_BANNER: &str = r#"
 ______     ______   ______     ______     __    __     __
/\  ___\   /\__  _\ /\  __ \   /\  == \   /\ "-./  \   /\ \
\ \___  \  \/_/\ \/ \ \ \/\ \  \ \  __<   \ \ \-./\ \  \ \ \
 \/\_____\    \ \_\  \ \_____\  \ \_\ \_\  \ \_\ \ \_\  \ \_\
  \/_____/     \/_/   \/_____/   \/_/ /_/   \/_/  \/_/   \/_/
"#;

#[tokio::main]
async fn main() {
    println!("{}", &ASCII_BANNER);
    println!("> Stormi is starting...");

    // TODO: Move configuration logging logic to a separate non-async function
    let config = ConfigSingletonReader::singleton()
        .inner
        .lock()
        .expect("Thread failed to unwrap `ConfigSingletonReader`");
    if config.is_default_user {
        println!("> Configuration file `config.yaml` is invalid or does not exist");
        println!("..This is highly insecure. Please consider adding valid configuration");
        println!(
            "..Defaulting to user `{}` with default permissions: {:?}",
            &DEFAULT_USERNAME, &DEFAULT_PERMISSIONS
        );
    } else {
        println!(
            "> Configuration loaded. Detected {} user(s)",
            &config.get_users().len()
        )
    }

    // Dropping the variable so that other threads
    // have no problems with accessing the singleton
    drop(config);

    // Getting a custom port from the environment or binding to the default one
    let port = match env::var("STORMI_PORT") {
        Ok(port) => port.parse::<u16>().unwrap_or_else(|_| {
            eprintln!(
                "`STORMI_PORT` should be a 16-bit unsigned integer. {} is out of range",
                &port
            );
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
        .route_layer(extractor_middleware::<Authentication>())
        // TODO: Add authentication layer
        .route("/remove", delete(remove::handler))
        .route_layer(extractor_middleware::<Authentication>())
        .fallback(not_found::handler.into_service());

    let server = axum::Server::bind(&addr).serve(stormi.into_make_service());

    println!("> Stormi started at {}", &addr);
    if let Err(error) = server.await {
        eprintln!("{}", &error);
    }
}
