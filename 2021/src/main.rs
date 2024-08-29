use clap::Parser;
use days::{day1, day2, day3, day7};

mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day7;
}

mod helpers {
    pub mod data_structs;
    pub mod file;
}

#[derive(Parser)]
#[command(name = "aoc")]
#[command(version = "1.0")]
#[command(about = "aoc", long_about = None)]
struct Cli {
    /// day
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let cli = Cli::parse();

    match cli.day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        7 => day7::solve(),

        _ => unreachable!(),
    }
}
