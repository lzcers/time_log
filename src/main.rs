mod app;
mod database;
mod tag;
mod time;
mod utils;

use app::AppHandle;
use clap::{Parser, Subcommand};
use database::Database;
use utils::display_timer_status;

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
        /// Optional arguments that can be duration (e.g. "5m") or tags
        #[arg(required = false)]
        args: Vec<String>,
    },
    #[clap(alias = "t")]
    Stop,
    /// Show current status
    #[clap(alias = "c")]
    Current,
    /// Exit the program
    Exit,
}

fn parse_duration(s: &str) -> Result<u64, String> {
    if s.is_empty() {
        return Err("Duration string cannot be empty".to_string());
    }

    // 纯数字视为秒
    if s.chars().all(|c| c.is_digit(10)) {
        return s
            .parse::<u64>()
            .map(|seconds| seconds * 1000)
            .map_err(|_| format!("Invalid number: '{}'", s));
    }

    // 解析带单位的时间
    let (num_str, unit) = s.split_at(s.len() - 1);
    let num = num_str
        .parse::<u64>()
        .map_err(|_| format!("Invalid number: '{}'", num_str))?;

    match unit {
        "s" => Ok(num * 1000),      // 秒转毫秒
        "m" => Ok(num * 60 * 1000), // 分钟转毫秒
        _ => Err(format!(
            "Invalid time unit: '{}'. Expected 's' or 'm'",
            unit
        )),
    }
}

fn parse_start_args(args: Vec<String>) -> (Option<u64>, Vec<String>) {
    if args.is_empty() {
        return (None, vec![]);
    }

    // 检查第一个参数是否是时间格式
    if let Ok(duration) = parse_duration(&args[0]) {
        // 如果第一个参数是时间，剩余的都是标签
        (Some(duration), args[1..].to_vec())
    } else {
        // 如果第一个参数不是时间，所有参数都作为标签
        (None, args)
    }
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
        // println!("get command: {:?}", cli.command);
        println!("");
        match cli.command {
            Commands::Start { args } => {
                let (duration, tags) = parse_start_args(args);
                if let Err(e) = app_handle.start_timer(duration, tags) {
                    println!("{}", e.to_string());
                    if let Ok(status) = app_handle.get_current_timer_status() {
                        display_timer_status(&status);
                    }
                }
            }
            Commands::Stop => {
                if let Ok(status) = app_handle.stop_timer() {
                    display_timer_status(&status);
                }
            }
            Commands::Current => {
                if let Ok(status) = app_handle.get_current_timer_status() {
                    display_timer_status(&status);
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
