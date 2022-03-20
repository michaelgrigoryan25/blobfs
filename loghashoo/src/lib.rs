use std::io::Write;

use chrono::Local;
use colored::{Color, Colorize};
use log::{Level, Log};

/// Custom logger implementation for Hashoo, for fast and easy logging.
pub struct Logger;

impl Logger {
    /// This function initializes [Logger] globally.
    pub fn setup() -> Result<(), log::SetLoggerError> {
        log::set_logger(&Logger)
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() == Level::Info
    }

    fn log(&self, record: &log::Record) {
        // Outputting current date and time and opening a square bracket,
        // which will contain the level of logging.
        print!("{} [", Local::now().to_string().color(Color::BrightBlack));

        let level = record.level().as_str();
        // Coloring the text inside of the brackets based on the record level.
        match record.level() {
            Level::Warn => print!("{}", level.color(Color::Yellow)),
            Level::Info => print!("{}", level.color(Color::BrightBlue)),
            Level::Error => print!("{}", level.color(Color::BrightRed)),
            Level::Trace => print!("{}", level.color(Color::BrightWhite)),
            Level::Debug => print!("{}", level.color(Color::BrightBlack)),
        }

        print!("] > ");
        println!("{}", record.args());
    }

    fn flush(&self) {
        std::io::stdout().flush().unwrap();
    }
}
