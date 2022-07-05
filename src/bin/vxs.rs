use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	match vxs::cmd::VxCommand::from_args() {
		vxs::cmd::VxCommand::Start(args) => vxs::bootstrap(args).await?,
	}

	Ok(())
}
