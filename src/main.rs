use clap::{Parser, ValueEnum};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn read_battery_file(bat: &str, filename: &str) -> Result<String, Box<dyn Error>> {
    let path = PathBuf::from(format!("/sys/class/power_supply/{}/{}", bat, filename));
    Ok(fs::read_to_string(&path)?.trim().to_string())
}

fn percent(bat: &str) -> Result<u8, Box<dyn Error>> {
    let content = read_battery_file(bat, "capacity")?;
    Ok(content.parse()?)
}

fn status(bat: &str) -> Result<char, Box<dyn Error>> {
    let content = read_battery_file(bat, "status")?;
    Ok(match content.as_str() {
        "Discharging" => '-',
        "Charging" => '+',
        _ => '?',
    })
}

fn read_energy(bat: &str, energy_kind: &str) -> Result<u64, Box<dyn Error>> {
    let content = read_battery_file(bat, energy_kind)?;
    Ok(content.parse()?)
}

fn percent_combined(bats: &[String]) -> Result<u8, Box<dyn Error>> {
    let (mut full_sum, mut now_sum) = (0u64, 0u64);

    for bat in bats {
        let full = read_energy(bat, "energy_full")?;
        let now = read_energy(bat, "energy_now")?;

        full_sum += full;
        now_sum += now;
    }

    if full_sum == 0 {
        Err("Total full energy is zero".into())
    } else {
        Ok(((now_sum as f64 / full_sum as f64) * 100.0) as u8)
    }
}

/// Simple program to get battery stats
#[derive(Parser, Debug)]
#[command(name = "pwr_stats", version, about)]
struct Args {
    /// Battery name(s) in /sys/class/power_supply
    #[arg(long, short, default_values_t = vec!["BAT1".to_owned()])]
    bats: Vec<String>,

    /// Info to be displayed
    #[arg(long, short, default_value_t = Info::All, value_enum)]
    info: Info,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Info {
    Percent,
    PercentCombined,
    Status,
    All,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.info {
        Info::PercentCombined => {
            println!("{}", percent_combined(&args.bats)?);
        }
        Info::Percent => {
            for bat in &args.bats {
                println!("{}", percent(bat)?);
            }
        }
        Info::Status => {
            for bat in &args.bats {
                println!("{}", status(bat)?);
            }
        }
        Info::All => {
            for bat in &args.bats {
                println!("{}{}%", status(bat)?, percent(bat)?);
            }
        }
    }

    Ok(())
}
