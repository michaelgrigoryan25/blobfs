use crate::config::Config;
use std::{env, net::SocketAddr};

const DEFAULT_PORT: u16 = 6345;
/// Default host for Stormi
const LOCALHOST: &str = "127.0.0.1";

/// This will try to get the port from `config.yaml`.
/// If the port is [None], we will try to get and parse
/// the `STORMI_PORT` environment variable. If it is [None]
/// too then we will default to `6345`.
pub fn get_port(config: &Config) -> u16 {
    if let Some(port) = config.port {
        port
    } else {
        if let Ok(port) = env::var("STORMI_PORT") {
            port.parse::<u16>().expect(&format!(
                "`STORMI_PORT` should be a 16-bit unsigned integer. {} is out of range",
                port
            ))
        } else {
            DEFAULT_PORT
        }
    }
}

/// This will attempt to get the address provided
/// from the `config.yaml` file. If it does not exist
/// in the configuration then we will default to [LOCALHOST]
pub fn get_addr(config: &Config, port: u16) -> SocketAddr {
    format!(
        "{}:{}",
        if let Some(addr) = &config.addr {
            addr
        } else {
            LOCALHOST
        },
        port
    )
    .parse()
    .expect("Invalid `SocketAddr` was supplied")
}
