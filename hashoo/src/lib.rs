use clap::Parser;
use log::{error, info, LevelFilter};

pub const HASHOO_BANNER: &str = r#"
                                               .-'''-.        .-'''-.     
                                               '   _    \     '   _    \   
    .                                .       /   /` '.   \  /   /` '.   \  
  .'|                              .'|      .   |     \  ' .   |     \  '  
 <  |                             <  |      |   '      |  '|   '      |  ' 
  | |             __               | |      \    \     / / \    \     / /  
  | | .'''-.   .:--.'.         _   | | .'''-.`.   ` ..' /   `.   ` ..' /   
  | |/.'''. \ / |   \ |      .' |  | |/.'''. \  '-...-'`       '-...-'`    
  |  /    | | `" __ | |     .   | /|  /    | |                             
  | |     | |  .'.''| |   .'.'| |//| |     | |                             
  | |     | | / /   | |_.'.'.-'  / | |     | |                             
  | '.    | '.\ \._,\ '/.'   \_.'  | '.    | '.                            
  '---'   '---'`--'  `"            '---'   '---'                            "#;

/// This function is responsible for doing initialization tasks
/// required for Hashoo.
#[doc(hidden)]
pub fn init() -> anyhow::Result<()> {
    println!("{HASHOO_BANNER}");

    // Setting the maximum logging level for the logger, so that
    // irrelevant things do not get printed out in stdout.
    log::set_max_level(LevelFilter::Info);

    // Setting the default logger for the log library to our custom logger,
    // which is simple, and has nice formatting.
    loghashoo::Logger::setup().unwrap();

    info!("Hashoo server is starting...");

    // Parsing the arguments supplied from the CLI and validating them
    // using the `clap` crate.
    let args = Args::parse();

    // Checking if the configuration exists at the specified path. If it
    // does not, the process will exit with a status code of `1`.
    if let Err(_) = std::fs::metadata(&args.config) {
        error!("configuration was not found at: `{}`", &args.config);

        // End the process with a failure
        std::process::exit(1);
    }

    if let Err(_) = std::fs::metadata(&args.destination) {
        // Creating the destination location if it does not exist already.
        std::fs::create_dir(args.destination)?;
        info!("destination directory has been created by hashoo...")
    }

    Ok(())
}

/// CLI arguments that will be passed to Hashoo on start. These will
/// be useful when configuring the server.
#[derive(Parser, Debug)]
pub struct Args {
    /// Listening address for Hashoo. Default address is `127.0.0.1:6375`
    #[clap(long, default_value = "127.0.0.1:6375")]
    pub addr: String,

    /// Path to the configuration file of Hashoo. Default search path
    /// is `./.conf.yaml`.
    #[clap(long, default_value = "./.conf.yaml")]
    pub config: String,

    /// Upload location for keeping the files sent to Hashoo. Default destination
    /// for uploads is `./.hashoodata`.
    #[clap(long, default_value = "./.hashoodata")]
    pub destination: String,
}
