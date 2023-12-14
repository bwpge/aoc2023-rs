use std::process::ExitCode;

use anyhow::anyhow;
use aoc::{cli::Cli, error};
use clap::Parser;

fn main() -> ExitCode {
    let args = Cli::parse();
    let input = args
        .input
        .unwrap_or_else(|| format!("data/day{}.txt", { args.day }).into());

    if !(input.exists() && input.is_file()) {
        error!("input '{}' does not exist", input.display());
        return ExitCode::FAILURE;
    }

    let result = match args.day {
        1 => aoc::day1::exec(input),
        2 => aoc::day2::exec(input),
        _ => Err(anyhow!("no solution found for day {}", args.day)),
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            error!("{err}");
            ExitCode::FAILURE
        }
    }
}
