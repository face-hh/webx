mod cli;
mod config;
mod http;
mod kv;
mod secret;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{LogLevel, Verbosity};
use config::Config;
use macros_rs::fs::file_exists;

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
    #[arg(global = true, short, long, default_value_t = String::from("config.toml"), help = "config path")]
    config: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the daemon
    Start,
    /// Manage API keys
    Key {
        #[command(subcommand)]
        command: Key,
    },
}

#[derive(Subcommand)]
enum Key {
    /// List all keys
    #[command(visible_alias = "ls")]
    List,
    /// Export all keys
    #[command(visible_alias = "save")]
    Export {
        /// Exported file name
        filename: String,
    },
    /// Get API key info
    #[command(visible_alias = "i")]
    Info {
        /// Key name
        name: String,
    },
    /// Create privileged API key
    #[command(visible_alias = "mk")]
    Create {
        /// Key name
        name: String,
    },
    /// Remove API key
    #[command(visible_alias = "rm")]
    Delete {
        /// Key name
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut env = pretty_env_logger::formatted_builder();
    let level = cli.verbose.log_level_filter();

    env.filter_level(level).init();

    if !file_exists!(&cli.config) {
        Config::new().set_path(&cli.config).write();
        log::warn!("Written initial config, please add MongoDB details");
        std::process::exit(1);
    }

    match &cli.command {
        Commands::Start => {
            if let Err(err) = http::start(cli) {
                log::error!("Failed to start server: {err}")
            }
        }
        Commands::Key { command } => match command {
            Key::List => cli::list(&cli),
            Key::Info { name } => cli::info(&cli, name),
            Key::Create { name } => cli::create(&cli, name),
            Key::Delete { name } => cli::remove(&cli, name),
            Key::Export { filename } => cli::export(&cli, filename),
        },
    };
}
