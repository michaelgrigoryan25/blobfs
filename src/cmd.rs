use structopt::StructOpt;

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
