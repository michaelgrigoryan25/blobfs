use structopt::StructOpt;

// making structopt publicly available so that we don't have to install
// it just to access the traits and parse the arguments.
pub use structopt;

#[derive(Debug, StructOpt)]
pub struct VxsArgs {
    #[structopt(short, long)]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub cmd: VxCommand,
}

#[derive(Debug, StructOpt)]
pub enum VxCommand {
    /// Start vxs-server
    Start(VxCommandStartArgs),
    // /// Stop vxs-server
    // Stop(VxCommandStopArgs),
}

// #[derive(Debug, StructOpt)]
// pub struct VxCommandStopArgs {}

#[derive(Debug, StructOpt)]
pub struct VxCommandStartArgs {
    #[structopt(short, long)]
    pub address: String,
}
