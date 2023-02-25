use clap::Parser;
use simplelog::LevelFilter;
use std::io::BufRead;
use twitter_model::api::ApiObject;

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    init_logging(opts.verbose)?;

    match opts.command {
        Command::Validate => {
            let mut line_number = 0;
            let mut user_count = 0;
            let mut status_count = 0;
            let mut search_results_count = 0;
            let mut deletion_count = 0;
            let mut error_count = 0;
            let lines = std::io::stdin().lock().lines();

            for line in lines {
                let line = line?;
                line_number += 1;

                match serde_json::from_str::<ApiObject>(&line) {
                    Ok(api_object) => match api_object {
                        ApiObject::User(_) => {
                            user_count += 1;
                        }
                        ApiObject::Status(_) => {
                            status_count += 1;
                        }
                        ApiObject::SearchResults(_) => {
                            search_results_count += 1;
                        }
                        ApiObject::Deletion(_) => {
                            deletion_count += 1;
                        }
                    },
                    Err(_error) => {
                        error_count += 1;
                        log::error!("Line: {}", line_number);
                    }
                }
            }

            log::info!("Errors: {error_count}");
            log::info!("Users: {user_count}");
            log::info!("Statuses: {status_count}");
            log::info!("Search results: {search_results_count}");
            log::info!("Deletions: {deletion_count}");
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(name = "twitter-model", version, author)]
struct Opts {
    /// Level of verbosity
    #[clap(short, long, global = true, action = clap::ArgAction::Count)]
    verbose: u8,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Validate,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Logging initialization error")]
    LogInit(#[from] log::SetLoggerError),
}

fn select_log_level_filter(verbosity: u8) -> LevelFilter {
    match verbosity {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }
}

fn init_logging(verbosity: u8) -> Result<(), log::SetLoggerError> {
    simplelog::TermLogger::init(
        select_log_level_filter(verbosity),
        simplelog::Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )
}
