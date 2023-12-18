use std::{process::ExitCode, time::Instant};

use anyhow::anyhow;
use aoc::{cli::Cli, error, solutions};
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
        1 => solutions::day01::exec(input),
        2 => solutions::day02::exec(input),
        3 => solutions::day03::exec(input),
        4 => solutions::day04::exec(input),
        5 => solutions::day05::exec(input),
        6 => solutions::day06::exec(input),
        7 => solutions::day07::exec(input),
        8 => solutions::day08::exec(input),
        9 => solutions::day09::exec(input),
        10 => solutions::day10::exec(input),
        11 => solutions::day11::exec(input),
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
