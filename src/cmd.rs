use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum VxCommand {
	#[structopt()]
	Start(VxCommandStartArgs),
}

#[derive(Debug, StructOpt)]
pub struct VxCommandStartArgs {
	#[structopt(short, long, env = "VXS_ADDRESS", default_value = "127.0.0.1:0")]
	/// Specific binding hostname.
	pub address: String,

	#[structopt(short, long)]
	/// Enables additional output for debugging.
	pub verbose: bool,

	#[structopt(short, long, default_value = "debug.log")]
	/// Set a custom logging destination.
	pub log_path: String,

	#[structopt(long, default_value = "1500")]
	/// Connection limit for current instance.
	pub conn_limit: u16,
}
