use std::{process::ExitCode, time::Instant};

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

    let start = Instant::now();
    let result = match args.day {
        1 => aoc::day1::exec(input),
        2 => aoc::day2::exec(input),
        3 => aoc::day3::exec(input),
        4 => aoc::day4::exec(input),
        5 => aoc::day5::exec(input),
        6 => aoc::day6::exec(input),
        7 => aoc::day7::exec(input),
        8 => aoc::day8::exec(input),
        9 => aoc::day9::exec(input),
        _ => Err(anyhow!("no solution found for day {}", args.day)),
    };

    let elapsed = humantime::format_duration(start.elapsed());
    println!("\nTotal runtime: {elapsed}");

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            error!("{err}");
            ExitCode::FAILURE
        }
    }
}
