use tokio::sync::broadcast;

#[macro_use]
extern crate log;

/// Contains various commands, their arguments, for working with the command-line.
pub mod cmd;
/// Core server code is included in this module.
mod server;

#[doc(hidden)]
/// This will be printed to stdout, when vxs is bootstrapped.
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

/// Bootstraps, and starts a new vxs server with the provided [cmd::VxCommandStartArgs].
pub async fn bootstrap(args: cmd::VxCommandStartArgs) -> Result<(), Box<dyn std::error::Error>> {
	print!("{}\n{}\n", VXS_BANNER, divider("vxs"));
	fern::Dispatch::new()
		.format(|out, message, record| {
			out.finish(format_args!(
				"{} ({}) {}",
				chrono::Local::now().format("%Y-%m-%d@%H:%M:%S"),
				record.level().to_string().to_lowercase(),
				message
			))
		})
		.chain(fern::log_file(args.log_dest)?)
		.chain(std::io::stdout())
		.level(log::LevelFilter::Trace)
		.apply()
		.expect("fern: cannot configure the logger");
	debug!("fern: logger configured successfully");

	let (tx, mut rx) = broadcast::channel::<()>(1);
	let server = server::Server::new(args.address, tx.clone()).await?;
	debug!("tcp socket bound at: {}", server.listener.local_addr()?);

	tokio::select! {
		output = server.run() => {
			if let Err(e) = output {
				error!("tcp/ip server socket error: {}", e);
			}

			tx.send(())?;
		}

		_ = tokio::signal::ctrl_c() => {
			info!("ctrl+c captured. quitting...");
			tx.send(())?;
		}
	}

	Ok(rx.recv().await?)
}

#[inline(always)]
#[doc(hidden)]
/// There is no need pf creating a separate stack frame just for
/// this small function, thus, it can be inlined.
fn divider(msg: &str) -> String {
	format!("{d} {m} {d}", d = "-".repeat(30), m = msg)
}
