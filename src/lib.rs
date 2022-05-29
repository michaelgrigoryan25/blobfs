pub mod cmd;
pub mod server;

pub type UnspecifiedError<T> = Result<T, Box<dyn std::error::Error>>;
