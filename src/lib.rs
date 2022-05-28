pub mod cmd;
pub mod graceful;
pub mod server;

pub type UnspecifiedError<T> = Result<T, Box<dyn std::error::Error>>;
