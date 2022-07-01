use structopt::StructOpt;
use vxs::Unspecified;

#[tokio::main]
async fn main() -> Unspecified<()> {
    match vxs::cmd::VxsArgs::from_args().cmd {
        vxs::cmd::VxCommand::Start(args) => vxs::bootstrap(args).await?,
    }

    Ok(())
}
