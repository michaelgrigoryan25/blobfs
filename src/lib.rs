#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

/// Contains various commands, their arguments, for working with the command-line.
pub mod cmd;
/// Core server code is included in this module.
mod server;

#[doc(hidden)]
const VXS_BANNER: &str = r#"
 ___      ___ ___    ___ ________
|\  \    /  /|\  \  /  /|\   ____\
\ \  \  /  / | \  \/  / | \  \___|_
 \ \  \/  / / \ \    / / \ \_____  \
  \ \    / /   /     \/   \|____|\  \
   \ \__/ /   /  /\   \     ____\_\  \
    \    /   /__/ /\ __\   |\_________\
     \__/    |__|/ \|__|   \|_________|
"#;

#[doc(hidden)]
const LOGGING_TIME_FORMAT: &str = "%Y-%m-%d@%H:%M:%S";

/// Bootstraps, and starts a new vxs server with the provided [cmd::VxCommandStartArgs].
pub async fn bootstrap(args: cmd::VxCommandStartArgs) -> Result<(), Box<dyn std::error::Error>> {
	print!("{}\n{}\n", VXS_BANNER, divider("vxs"));

	fern::Dispatch::new()
		.format(|out, message, record| {
			out.finish(format_args!(
				"{} ({}) {}",
				chrono::Local::now().format(LOGGING_TIME_FORMAT),
				record.level().to_string().to_lowercase(),
				message
			))
		})
		.chain(fern::log_file(args.log_path)?)
		.chain(std::io::stdout())
		.level(log::LevelFilter::Trace)
		.apply()
		.expect("fern: cannot configure the logger");
	debug!("fern: logger configured successfully");

	let server = server::Server::new(args.address).await?;
	debug!("tcp/ip socket bound at: {}", server.listener.local_addr()?);

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
fn divider(msg: &str) -> String {
	format!("{d} {m} {d}", d = "-".repeat(30), m = msg)
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
