use structopt::StructOpt;
use vxs::Unspecified;

#[tokio::main]
async fn main() -> Unspecified<()> {
    match vxs::cmd::VxCommand::from_args() {
        vxs::cmd::VxCommand::Start(args) => vxs::bootstrap(args).await?,
    }

    Ok(())
}
