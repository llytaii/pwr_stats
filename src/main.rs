use std::fs;

use clap::{Parser, ValueEnum};

// percentage
fn print_percentage(bat: &str) {
    let path = format!("/sys/class/power_supply/{}/capacity", bat);
    match fs::read_to_string(&path) {
        Ok(content) => {
            println!("{}", content);
        }
        Err(e) => {
            eprintln!("error: {}", e);
        }
    }
}

// charging / discharging
fn print_status(bat: &str) {
    let path = format!("/sys/class/power_supply/{}/status", bat);
    match fs::read_to_string(&path) {
        Ok(content) => {
            let content = match content.trim() {
                "Discharging" => "-",
                "Charging" => "+",
                _ => "?",
            };
            println!("{}", content);
        }
        Err(e) => {
            eprintln!("error: {}", e);
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
    #[arg(long, short, default_value_t = Info::Percentage, value_enum)]
    info: Info,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Info {
    Percentage,
    Status,
}

fn main() {
    let args = Args::parse();

    match args.info {
        Info::Percentage => print_percentage(&args.bat),
        Info::Status => print_status(&args.bat),
    }
}
