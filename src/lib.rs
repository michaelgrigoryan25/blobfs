pub mod cmd;
pub mod server;

/// Simple wrapper around [Result], where we don't care about the received
/// error type.
///
/// Note that this wrapper only works with errors which implement [std::error::Error].
pub type UnspecifiedError<T> = Result<T, Box<dyn std::error::Error>>;
