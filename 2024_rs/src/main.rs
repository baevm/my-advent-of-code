use clap::Parser;
use days::day1;

mod days {
    pub mod day1;
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

        _ => unreachable!(),
    }
}
