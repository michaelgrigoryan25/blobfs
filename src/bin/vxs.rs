use structopt::StructOpt;
use tokio::net::TcpListener;
use tokio::signal;
use vxs::server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	match vxs::cmd::VxCommand::from_args() {
		vxs::cmd::VxCommand::Start(args) => {
			if let Err(err) = vxs::clog(args.log_path.clone()) {
				eprintln!("fern configuration error: {}", err);
			}

			match TcpListener::bind(&args.address).await {
				Ok(listener) => server::run(listener, args, signal::ctrl_c()).await,
				Err(err) => {
					log::set_max_level(log::LevelFilter::Off);
					eprintln!("tcp/ip socket error: {}", err);
				}
			}
		}
	}

	Ok(())
}
