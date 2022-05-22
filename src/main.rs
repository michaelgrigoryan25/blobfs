use vxs_cli::{helpers::proc, structopt::StructOpt, VxCommand, VxsArgs};
use vxs_server::VxServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vxs = VxsArgs::from_args();
    match vxs.cmd {
        VxCommand::Start(args) => {
            // TODO: Create a server, and bind to the address from args.
            let _ = VxServer::create(args);
            // TODO: Start the server in detached or attached mode based on args.
        }

        VxCommand::Proc(args) => proc::list(args).await?,
    }

    Ok(())
}
