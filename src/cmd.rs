use structopt::StructOpt;

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
}

#[derive(Debug, StructOpt)]
pub struct VxCommandStartArgs {
    #[structopt(short, long, env = "VXS_ADDRESS")]
    /// Binding address for vxs-server. Should contain a valid
    /// socket address. This flag must contain a valid IP address,
    /// or hostname.
    pub address: Option<String>,

    #[structopt(short, long, env = "VXS_DETACHED")]
    /// Detached indicates, whether the user spawned the server in the background
    /// or started it in the foreground. This is passed as an argument from the
    /// CLI.
    pub detached: bool,
}
