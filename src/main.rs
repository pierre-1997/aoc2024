use clap::{Parser, Subcommand};

mod day1;
mod day2;
mod day3;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    All,
    Day { day: usize },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::All => {
            todo!()
        }
        Commands::Day { day } => {
            let d1p1 = day1::part_1(include_str!("../inputs/d1"));
            let d1p2 = day1::part_2(include_str!("../inputs/d1"));
            let d2p1 = day2::part_1(include_str!("../inputs/d2"));
            let d2p2 = day2::part_2(include_str!("../inputs/d2"));
            let d3p1 = day3::part_1(include_str!("../inputs/d3"));
            let d3p2 = day3::part_2(include_str!("../inputs/d3"));

            dbg!(d3p2);

            todo!()
        }
    }
}
