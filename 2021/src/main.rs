use clap::Parser;
use days::{day1, day10, day14, day2, day3, day4, day6, day7, day9};

mod days {
    pub mod day1;
    pub mod day10;
    pub mod day14;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day6;
    pub mod day7;
    pub mod day9;
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
        4 => day4::solve(),
        6 => day6::solve(),
        7 => day7::solve(),
        9 => day9::solve(),
        10 => day10::solve(),
        14 => day14::solve(),

        _ => unreachable!(),
    }
}
