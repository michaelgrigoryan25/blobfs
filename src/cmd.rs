//! Contains various commands, their arguments, for working with the command-line.

use structopt::StructOpt;

#[derive(StructOpt)]
pub enum VxCommand {
	#[structopt()]
	Start(VxCommandStartArgs),
}

#[derive(StructOpt)]
pub struct VxCommandStartArgs {
	#[structopt(short, long, env = "VXS_ADDRESS", default_value = "127.0.0.1:0")]
	pub address: String,

	#[structopt(short, long)]
	pub verbose: bool,

	#[structopt(long, default_value = "debug.log")]
	pub log_path: String,

	#[structopt(long, default_value = "1500")]
	pub max_connections: usize,
}
