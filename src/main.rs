use std::fs::read_to_string;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

mod days;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Day01 { input_path: String },
    Day02 { input_path: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Day01 { input_path } => {
            use days::day01::*;
            let input = read_to_string(input_path)?;
            println!("Part 1: {}", part1(&input)?);
            println!("Part 2: {}", part2(&input)?);
        }
        Command::Day02 { input_path } => {
            use days::day02::*;
            let input = read_to_string(input_path)?;
            println!("Part 1: {}", part1(&input)?);
            println!("Part 2: {}", part2(&input)?);
        }
    };

    Ok(())
}
