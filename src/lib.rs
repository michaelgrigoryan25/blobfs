#[macro_use]
extern crate log;

pub mod cmd;
pub(crate) mod server;

pub async fn bootstrap(args: cmd::VxCommandStartArgs) -> Result<(), Box<dyn std::error::Error>> {
	configure_logging(&args)?;
	let (shutdown_notifier, _) = tokio::sync::broadcast::channel::<()>(1);
	let server = server::Server::new(args.address, args.max_connections, shutdown_notifier).await?;

	tokio::select! {
		output = server.run() => {
			if let Err(e) = output {
				error!("tcp/ip server socket error: {}", e);
			}
		}

		_ = tokio::signal::ctrl_c() => {
			info!("ctrl+c captured. quitting...");
		}
	}

	Ok(())
}

#[inline(always)]
#[doc(hidden)]
/// There is no need pf creating a separate stack frame just for
/// this small function, thus, it can be inlined.
fn _divider(msg: &str) -> String {
	format!("{d} {m} {d}", d = "-".repeat(30), m = msg)
}

fn configure_logging(args: &cmd::VxCommandStartArgs) -> Result<(), Box<dyn std::error::Error>> {
	fern::Dispatch::new()
		.format(|out, message, record| {
			out.finish(format_args!(
				"{} ({}) {}",
				chrono::Local::now().format("%Y-%m-%d@%H:%M:%S"),
				record.level().to_string().to_lowercase(),
				message
			))
		})
		.chain(fern::log_file(args.log_path.clone())?)
		.chain(std::io::stdout())
		.level(log::LevelFilter::Trace)
		.apply()
		.expect("fern: cannot configure the logger");
	Ok(())
}

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
