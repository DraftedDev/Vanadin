#[macro_use]
extern crate log;

use clap::{Arg, ArgAction, Command};
use log::{Level, LevelFilter};
use simplelog::{Color, ColorChoice, ConfigBuilder, TerminalMode};

pub mod cli;
pub mod vanadir;

fn main() {
    simplelog::TermLogger::init(
        {
            let var = std::env::var("LOG_LEVEL").unwrap_or("INFO".to_string());
            match var.as_str() {
                "DEBUG" => LevelFilter::Debug,
                "INFO" => LevelFilter::Info,
                "WARN" => LevelFilter::Warn,
                "ERROR" => LevelFilter::Error,
                _ => LevelFilter::Info,
            }
        },
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Debug)
            .set_level_color(Level::Info, Some(Color::Green))
            .set_level_color(Level::Warn, Some(Color::Yellow))
            .set_level_color(Level::Error, Some(Color::Red))
            .set_level_color(Level::Debug, Some(Color::Cyan))
            .set_level_color(Level::Trace, Some(Color::Magenta))
            .build(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .expect("Failed to initialize logger");

    let matches = Command::new("vanadin")
        .subcommand_required(true)
        .author("DraftedDev")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommands([
            Command::new("info").about("Shows information about the local Vanadin installation"),
            Command::new("init")
                .about("Initializes a basic .vanadin folder for the current project"),
            Command::new("execute")
                .aliases(["x", "exec", "run"])
                .about("Executes the specified task")
                .args([Arg::new("task")
                    .action(ArgAction::Set)
                    .short('t')
                    .help("The task to execute")
                    .required(true)]),
        ])
        .get_matches();

    if let Some((cmd, args)) = matches.subcommand() {
        match cmd {
            "info" => cli::info::info(),
            "init" => cli::init::init(),
            "execute" => cli::execute::execute(args),
            _ => (),
        }
    }
}
