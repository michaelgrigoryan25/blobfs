use std::{env, fs::File, process::exit};

use simplelog::{CombinedLogger, Config, TermLogger, WriteLogger};

pub mod crypto;
pub mod fsx;

// For reading files from the `data` folder
pub fn get_string_path(path_segments: &[&str]) -> String {
    let mut current_dir = env::current_dir().unwrap();

    for &segment in path_segments {
        current_dir.push(segment);
    }

    current_dir.to_string_lossy().to_string()
}

pub fn init_loggers() -> Result<(), log::SetLoggerError> {
    // The path of the log file
    let log_path = get_string_path(&["data", "logs", "stormi.log"]);

    // Initializing the loggers
    CombinedLogger::init(vec![
        TermLogger::new(
            simplelog::LevelFilter::Info,
            Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        WriteLogger::new(
            simplelog::LevelFilter::Off,
            Config::default(),
            File::create(&log_path).unwrap_or_else(|_| {
                error!("Cannot create `stormi.log` at {}", &log_path);
                exit(1)
            }),
        ),
    ])
}
