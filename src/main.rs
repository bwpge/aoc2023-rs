use std::process::ExitCode;

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

    match args.day {
        // TODO: add solution dispatch
        _ => {
            error!("no solution found for day {}", args.day);
            ExitCode::FAILURE
        }
    }
}
