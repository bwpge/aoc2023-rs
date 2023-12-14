//! Comand line argument parsing infrastructure.

use std::path::PathBuf;

use clap::Parser;

static NAME: &str = env!("CARGO_PKG_NAME");
static ABOUT: &str = "Run Advent of Code 2023 solutions.";
static VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA"),
    " ",
    env!("VERGEN_BUILD_DATE"),
    ")"
);
static HELP_TEMPLATE: &str = "{bin} {version}\n{author-with-newline}{about-section}\n{all-args}";

/// Argument parser for this project's command line interface.
#[derive(Debug, Parser)]
#[command(
    name = NAME,
    author,
    about = ABOUT,
    version = VERSION,
    help_template = HELP_TEMPLATE
)]
pub struct Cli {
    /// Specify which solution to run (e.g., `day<N>` or a number 1-25)
    #[arg(value_parser = parse_solution_day)]
    pub day: i32,
    /// Input file for the solution (default: `data/day<N>.txt`)
    #[arg(short, long)]
    pub input: Option<PathBuf>,
}

fn parse_solution_day(value: &str) -> Result<i32, String> {
    let s = value.strip_prefix("day").unwrap_or(value).trim();
    if let Ok(num) = s.parse::<i32>() {
        if (1..=25).contains(&num) {
            return Ok(num);
        } else {
            return Err(format!("{} is not in the range 1-25", num));
        }
    }

    Err("argument is not a valid solution name".into())
}
