use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long, default_value = "127.0.0.1:6375")]
    pub addr: String,

    #[clap(long, default_value = "./.conf.yaml")]
    pub config: String,

    #[clap(long)]
    pub destination: String,
}
