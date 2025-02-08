mod app;
mod database;
mod tag;
mod time;
mod utils;

use app::{App, AppHandle};
use clap::{Parser, Subcommand};
use database::Database;

#[derive(Parser)]
#[command(no_binary_name = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start a timer
    Start {
        /// Duration in minutes (e.g. "15m")
        #[arg(
            required = true,
            value_parser = parse_duration
        )]
        duration: u64,

        /// Optional tags (space separated)
        #[arg(required = false)]
        tags: Vec<String>,
    },
    /// Stop current timer
    Stop,
    /// Show current status
    Status,
    /// Exit the program
    Exit,
}

fn parse_duration(s: &str) -> Result<u64, String> {
    s.strip_suffix('m')
        .and_then(|num| num.parse().ok())
        .ok_or_else(|| {
            format!(
                "Invalid duration format: '{}'. Expected format like '15m'",
                s
            )
        })
}
fn main() -> anyhow::Result<()> {
    let app_handle = AppHandle::new(App::new(Database::new("akashic_log.db")?));

    loop {
        let input = dialoguer::Input::<String>::new()
            .with_prompt("akashic_log")
            .interact_text()?;

        let args = shell_words::split(&input)?;

        let cli = match Cli::try_parse_from(args) {
            Ok(cli) => cli,
            Err(e) => {
                println!("Error: {e}");
                println!("Available commands:");
                println!("  start [duration] [--tag TAG]");
                println!("  stop");
                println!("  status");
                println!("  exit");
                continue;
            }
        };
        println!("get command: {:?}", cli.command);
        match cli.command {
            Commands::Start { duration, tags } => {
                app_handle.start_timer(Some(duration), tags);
            }
            Commands::Stop => {
                app_handle.stop_timer()?;
            }
            Commands::Status => {
                println!("Current status:");
                // TODO: Implement status display
            }
            Commands::Exit => {
                println!("Exiting...");
                break;
            }
        }
    }

    Ok(())
}
