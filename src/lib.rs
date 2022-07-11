#[macro_use]
extern crate log;

pub mod cmd;
pub mod server;
mod shutdown;

/// [FromResulting] is similar to [From] trait, with a difference
/// being the [Result] return type. This trait provides a more
/// flexible way of handling conversion from type [T], and returning
/// [E] as an error.
pub trait FromResulting<T, E>
where
	Self: Sized,
{
	fn from_resulting(_: T) -> Result<Self, E>;
}

/// [IntoResulting] is similar to [Into] trait, with a difference
/// being the [Result] return type. This trait provides a more
/// flexible way of handling conversion into the type [T], and returning
/// [E] as an error.
pub trait IntoResulting<T, E>
where
	Self: Sized,
{
	fn into_resulting(self) -> Result<T, E>;
}

/// Configures logging with [fern].
///
/// Accepts the path to the logging output. By default, the value will
/// be handled by the `cmd` module of this crate, and if no other path
/// is provided, this will default to `./debug.log`.
///
/// # Errors
///
/// Returns [Err] if the provided log file output directory does not exist
/// on the system, or if the setup process fails in any way.
pub fn clog(log_path: String) -> Result<(), Box<dyn std::error::Error>> {
	fern::Dispatch::new()
		.format(|out, message, record| {
			out.finish(format_args!(
				"{} ({}) {}",
				chrono::Local::now().format("%Y-%m-%d@%H:%M:%S"),
				record.level().to_string().to_lowercase(),
				message
			))
		})
		.chain(fern::log_file(log_path)?)
		.chain(std::io::stdout())
		.level(log::LevelFilter::Trace)
		.apply()?;

	Ok(())
}
