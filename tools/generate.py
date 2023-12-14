#!/usr/bin/env python

import argparse, subprocess, sys
from pathlib import Path

SOLUTION_TEMPLATE = """//! Solution for Advent of Code 2023, Day {day}.
//!
//! # Day {day}{title}
//!
//! ## Part One
//!
//! ## Part Two
//!

use std::{{path::Path, process::ExitCode}};

fn part1() {{
    println!("Part 1: TODO");
}}

fn part2() {{
    println!("Part 1: TODO");
}}

pub fn exec<P: AsRef<Path>>(_path: P) -> ExitCode {{
    part1();
    part2();
    unimplemented!()
}}
"""


def bail(msg: str, code=1):
    print(f"error: {msg}")
    sys.exit(code)


def confirm(prompt: str | None = None):
    if prompt is None:
        prompt = "Are you sure you want to continue?"
    choice = input(f"{prompt} [y/N] ").lower()
    return choice == "y" or choice == "yes"


def status(msg: str, level: int = 1):
    indent = "  " * level
    print(f"{indent}- {msg}")


def update_file(path: Path, sentinel: str, before: str, day: int, mod_name: str):
    lines = []
    found = False
    with open(path, "r") as f:
        for line in f:
            s = line.strip()
            if s.startswith(sentinel):
                found = True
            if found and s.startswith(before):
                lines.append(f"        {day} => aoc::{mod_name}::exec(input),\n")
            lines.append(line)

    with open(path, "w", newline="\n") as f:
        f.writelines(lines)


parser = argparse.ArgumentParser(
    description="Generate an Advent of Code rust module for the given day.",
)
parser.add_argument(
    "-n",
    "--num",
    metavar="DAY",
    dest="day",
    type=int,
    required=True,
    help="the day number for this project",
)
parser.add_argument(
    "-d",
    "--parent-dir",
    metavar="DIR",
    type=str,
    help="specify a different parent directory for the project (default: ../)",
)
parser.add_argument(
    "-t",
    "--title",
    type=str,
    help="optional title for the day's problem (used in documentation)",
)
parser.add_argument(
    "--no-fmt",
    action="store_true",
    help="skip running `cargo fmt` after generating solution",
)


def validate_parent(path: Path):
    if not path.exists() or not path.is_dir():
        bail(f"parent directory `{path.resolve()}` does not exist")

    expected_files = [path.joinpath("Cargo.lock"), path.joinpath("Cargo.toml")]
    for f in expected_files:
        if not f.exists():
            bail(
                "parent directory is not suitable\n"
                f"  - expected to find `{f.name}` in `{f.parent.resolve()}`"
            )


def run_command(args: list[str]):
    status = subprocess.run(args, shell=True, check=False)
    if status.returncode != 0:
        bail(f"command `{' '.join(args)}` exited with status code {status}")


if __name__ == "__main__":
    args = parser.parse_args()

    parent_dir = Path(args.parent_dir or "..")
    validate_parent(parent_dir)

    day = args.day
    src_dir = parent_dir.joinpath("src")
    title = args.title or ""
    mod_name = f"day{day}"
    mod_file = src_dir.joinpath(f"{mod_name}.rs")
    main_file = src_dir.joinpath(f"main.rs")
    lib_file = src_dir.joinpath(f"lib.rs")

    if src_dir.is_file():
        bail("`src` path exists and is not a directory")
    if not src_dir.exists():
        bail("`src` directory does not exist")
    if mod_file.exists():
        print(f"WARNING: the module `{mod_file.name}` already exists.\n")
        if not confirm():
            sys.exit(0)

    print(f"\nGenerating solution:")
    status("Creating module")
    with open(mod_file, "w") as f:
        f.write(SOLUTION_TEMPLATE.format(day=day, title=f": {title}" if title else ""))

    status("Updating main.rs")
    update_file(main_file, "match args", "_ => {", day, mod_name)

    status("Updating lib.rs")
    with open(lib_file, "a+") as f:
        f.write(f"pub mod {mod_name};\n")

    if not args.no_fmt:
        status("Running `cargo fmt`")
        run_command(["cargo", "fmt"])

    print("\nSolution successfully created!")
    print(f"\nRemember to add the problem text to `{mod_name}.rs`.\n")
