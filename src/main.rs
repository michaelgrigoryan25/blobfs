use structopt::StructOpt;
use vxs::cmd::{proc, VxCommand, VxsArgs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vxs = VxsArgs::from_args();
    match vxs.cmd {
        VxCommand::Start(_) => {
            // TODO: Create a server, and bind to the address from args.
            // TODO: Start the server in detached or attached mode based on args.
        }

        VxCommand::Proc(args) => proc::list(args).await?,
    }

    Ok(())
}
