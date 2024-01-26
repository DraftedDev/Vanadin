use log::{Level, LevelFilter, SetLoggerError};
use simplelog::{Color, ColorChoice, ConfigBuilder, TerminalMode, ThreadLogMode};

use crate::types::LOG_LEVEL_ENV;

#[inline(always)]
pub fn init_logger() -> Result<(), SetLoggerError> {
    simplelog::TermLogger::init(
        {
            let var = std::env::var(LOG_LEVEL_ENV).unwrap_or("INFO".to_string());
            match var.to_uppercase().as_str() {
                "DEBUG" => LevelFilter::Debug,
                "INFO" => LevelFilter::Info,
                "WARN" => LevelFilter::Warn,
                "ERROR" => LevelFilter::Error,
                "TRACE" => LevelFilter::Trace,
                _ => LevelFilter::Trace,
            }
        },
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Debug)
            .set_location_level(LevelFilter::Off)
            .set_thread_mode(ThreadLogMode::Names)
            .set_target_level(LevelFilter::Off)
            .set_level_color(Level::Info, Some(Color::Cyan))
            .set_level_color(Level::Warn, Some(Color::Yellow))
            .set_level_color(Level::Error, Some(Color::Red))
            .set_level_color(Level::Debug, Some(Color::Green))
            .set_level_color(Level::Trace, Some(Color::Magenta))
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
}
