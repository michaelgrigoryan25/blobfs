use std::{env, fs::File, process::exit};

use simplelog::{CombinedLogger, Config, TermLogger, WriteLogger};

pub mod fsx;

pub fn get_log_path() -> String {
    let mut current_dir = env::current_dir().unwrap();
    let log_path_segments = &["data", "logs", "stormi.log"];

    for &segment in log_path_segments {
        current_dir.push(segment);
    }

    current_dir.to_string_lossy().to_string()
}

pub fn init_loggers() -> Result<(), log::SetLoggerError> {
    // The path of the log file
    let log_path = get_log_path();

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
