mod app;
mod clocker;
mod database;
mod description;
mod display;
mod tag;
mod time_slice;
mod timeline;
mod utils;

use app::AppHandle;
use clap::{Parser, Subcommand};
use database::Database;
use display::{display_current_timer_status, display_timer_sheet};
use utils::parse_start_args;

#[derive(Parser)]
#[command(no_binary_name = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(alias = "s")]
    Start {
        /// Duration in minutes (e.g. "15m")
        /// Optional arguments that can be duration (e.g. "5m") or description (e.g. "#code")
        #[arg(required = false)]
        args: Vec<String>,
    },
    #[clap(alias = "t")]
    Stop,
    /// Show current status
    #[clap(alias = "c")]
    Current,
    #[clap(alias = "l")]
    List,
    /// Exit the program
    Exit,
}

fn main() -> anyhow::Result<()> {
    let mut app_handle = AppHandle::new(Database::new("akashic_log.db")?);

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
        println!("");
        match cli.command {
            Commands::Start { args } => {
                let (duration, desc) = parse_start_args(args);
                if let Err(e) = app_handle.start_timer(duration, desc) {
                    println!("{}", e.to_string());
                    if let Ok(status) = app_handle.get_current_timer_status() {
                        display_current_timer_status(&status);
                    }
                }
            }
            Commands::Stop => {
                if let Ok(status) = app_handle.stop_timer() {
                    display_current_timer_status(&status);
                }
            }
            Commands::Current => {
                if let Ok(status) = app_handle.get_current_timer_status() {
                    display_current_timer_status(&status);
                }
            }
            Commands::List => {
                if let Ok(timeline) = app_handle.get_timeline(None, None, None) {
                    display_timer_sheet(&timeline);
                }
            }
            Commands::Exit => {
                println!("Exiting...");
                break;
            }
        }
        println!("");
    }

    Ok(())
}
