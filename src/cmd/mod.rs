use structopt::StructOpt;

pub mod proc;

#[derive(Debug, StructOpt)]
pub struct VxsArgs {
    #[structopt(short, long)]
    /// Similar to --verbose in other applications.
    ///
    // Not using the verbose flag, since structopt has a built-in flag --version,
    // which may create some confusion.
    pub debug: bool,

    #[structopt(subcommand)]
    pub cmd: VxCommand,
}

#[derive(Debug, StructOpt)]
pub enum VxCommand {
    /// Starts vxs, and binds it to the specified address.
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
    /// socket address. This flag must contain a valid IP address,
    /// or hostname.
    pub address: String,

    #[structopt(short, long)]
    /// Detached indicates, whether the user spawned the server in the background
    /// or started it in the foreground. This is passed as an argument from the
    /// CLI.
    pub detached: bool,
}
