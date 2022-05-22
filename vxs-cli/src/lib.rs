use structopt::StructOpt;

pub mod helpers;
pub use structopt;

#[derive(Debug, StructOpt)]
pub struct VxsArgs {
    #[structopt(subcommand)]
    pub cmd: VxCommand,
}

#[derive(Debug, StructOpt)]
pub enum VxCommand {
    /// Start vxs in your desired mode.
    Start(VxCommandStartArgs),

    /// Shows the list of processes running vxs.
    Proc(VxCommandProcArgs),
}

#[derive(Debug, StructOpt)]
pub struct VxCommandProcArgs {
    #[structopt(short, long)]
    /// Limits printing the process list only to `n` processes. If this
    /// argument is passed, the CLI will return only a partial list of
    /// processes running a vxs process.
    pub limit: Option<usize>,

    #[structopt(short, long)]
    /// Specify whether the printing process should continue infinitely
    /// unless cancelled by the user. If supplied, stdout will be flushed
    /// after each print.
    pub continuous: bool,

    #[structopt(short, long, default_value = "2")]
    /// Process list refresh interval in seconds. This is only usable when
    /// the `continuous` flag is supplied.
    pub interval: u64,
}

#[derive(Debug, StructOpt)]
pub struct VxCommandStartArgs {
    #[structopt(short, long, env = "VXS_ADDRESS")]
    /// Binding address for vxs-server. Should contain a valid
    /// socket address.
    pub address: String,

    #[structopt(short, long, env = "VXS_DETACHED")]
    /// Start vxs-server in detached mode, by spawning the process in
    /// the background.
    pub detached: bool,
}
