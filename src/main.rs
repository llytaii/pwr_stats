use std::fs;

use clap::{Parser, ValueEnum};

// percentage
fn percent(bat: &str) -> String {
    let path = format!("/sys/class/power_supply/{}/capacity", bat);
    match fs::read_to_string(&path) {
        Ok(content) => {
            format!("{}", content.trim())
        }
        Err(e) => {
            format!("{}", e)
        }
    }
}

// charging / discharging
fn status(bat: &str) -> String {
    let path = format!("/sys/class/power_supply/{}/status", bat);
    match fs::read_to_string(&path) {
        Ok(content) => {
            let content = match content.trim() {
                "Discharging" => "-",
                "Charging" => "+",
                _ => "?",
            };
            format!("{}", content.trim())
        }
        Err(e) => {
            format!("{}", e)
        }
    }
}

/// Simple program to get battery stats
#[derive(Parser, Debug)]
#[command(name = "pwr_stats", version, about, long_about = None)]
struct Args {
    /// battery name in /sys/class/power_supply
    #[arg(long, short, default_value_t = String::from("BAT1"))]
    bat: String,

    /// info to be displayed
    #[arg(long, short, default_value_t = Info::All, value_enum)]
    info: Info,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Info {
    Percent,
    Status,
    All,
}

fn main() {
    let args = Args::parse();

    match args.info {
        Info::Percent => print!("{}", percent(&args.bat)),
        Info::Status => print!("{}", status(&args.bat)),
        Info::All => print!("{}{}%", status(&args.bat), percent(&args.bat)),
    }
}
