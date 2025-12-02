use std::fs::read_to_string;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

mod days;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(short, long)]
    input_path: String,
}

#[derive(Subcommand)]
enum Command {
    Day01,
    Day02,
}

macro_rules! run_day {
    ($module:ident, $input_path:expr) => {{
        let input = read_to_string($input_path)?;
        println!("Part 1: {}", days::$module::part1(&input)?);
        println!("Part 2: {}", days::$module::part2(&input)?);
    }};
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Day01 => run_day!(day01, cli.input_path),
        Command::Day02 => run_day!(day02, cli.input_path),
    };

    Ok(())
}
