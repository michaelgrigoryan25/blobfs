use vxs_cli::{structopt::StructOpt, VxCommand, VxsArgs};
use vxs_server::VxServer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vxs = VxsArgs::from_args();
    match vxs.cmd {
        VxCommand::Start(args) => {
            let (address, verbose) = (args.address, vxs.verbose);
            let server = VxServer::new(address, verbose.into());
            unimplemented!();
        }
    }
}
