use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct VxsArgs {
    #[doc(hidden)]
    #[structopt(subcommand)]
    /// The executed sub-command. Sub-commands have the ability
    /// to include other sub-commands via `#[structopt()]`. This
    /// is just a way to group all of these commands together.
    pub cmd: VxCommand,
}

#[derive(Debug, StructOpt)]
pub enum VxCommand {
    #[structopt()]
    Start(VxCommandStartArgs),
}

#[derive(Debug, StructOpt)]
pub struct VxCommandStartArgs {
    #[structopt(short, long, env = "VXS_ADDRESS")]
    /// Specific binding hostname.
    pub address: Option<String>,

    #[structopt(short, long)]
    /// Enables additional output for debugging.
    pub verbose: bool,
}
