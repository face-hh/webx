mod config;
mod http;
mod secret;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{LogLevel, Verbosity};

#[derive(Copy, Clone, Debug, Default)]
struct Info;
impl LogLevel for Info {
    fn default() -> Option<log::Level> { Some(log::Level::Info) }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[clap(flatten)]
    verbose: Verbosity<Info>,
}

// add pmc restore command
#[derive(Subcommand)]
enum Commands {
    /// Start the daemon
    Start,
}

fn main() {
    let cli = Cli::parse();
    let mut env = pretty_env_logger::formatted_builder();
    let level = cli.verbose.log_level_filter();

    env.filter_level(level).init();

    match &cli.command {
        Commands::Start => {
            if let Err(err) = http::start() {
                log::error!("Failed to start server: {err}")
            }
        }
    };
}
