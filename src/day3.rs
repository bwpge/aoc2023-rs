//! Solution for Advent of Code 2023, Day 3.
//!
//! # Day 3: Gear Ratios
//!
//! ## Part One
//!
//! You and the Elf eventually reach a gondola lift station; he says the gondola
//! lift will take you up to the water source, but this is as far as he can
//! bring you. You go inside.
//!
//! It doesn't take long to find the gondolas, but there seems to be a problem:
//! they're not moving.
//!
//! "Aaah!"
//!
//! You turn around to see a slightly-greasy Elf with a wrench and a look of
//! surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working
//! right now; it'll still be a while before I can fix it." You offer to help.
//!
//! The engineer explains that an engine part seems to be missing from the
//! engine, but nobody can figure out which one. If you can add up all the part
//! numbers in the engine schematic, it should be easy to work out which part is
//! missing.
//!
//! The engine schematic (your puzzle input) consists of a visual representation
//! of the engine. There are lots of numbers and symbols you don't really
//! understand, but apparently any number adjacent to a symbol, even diagonally,
//! is a "part number" and should be included in your sum. (Periods (.) do not
//! count as a symbol.)
//!
//! Here is an example engine schematic:
//!
//! ```txt
//! 467..114..
//! ...*......
//! ..35..633.
//! ......#...
//! 617*......
//! .....+.58.
//! ..592.....
//! ......755.
//! ...$.*....
//! .664.598..
//! ```
//!
//! In this schematic, two numbers are not part numbers because they are not
//! adjacent to a symbol: 114 (top right) and 58 (middle right). Every other
//! number is adjacent to a symbol and so is a part number; their sum is 4361.
//!
//! Of course, the actual engine schematic is much larger. **What is the sum of
//! all of the part numbers in the engine schematic?**
//!
//! ## Part Two
//!
//! The engineer finds the missing part and installs it in the engine! As the
//! engine springs to life, you jump in the closest gondola, finally ready to
//! ascend to the water source.
//!
//! You don't seem to be going very fast, though. Maybe something is still
//! wrong? Fortunately, the gondola has a phone labeled "help", so you pick it
//! up and the engineer answers.
//!
//! Before you can explain the situation, she suggests that you look out the
//! window. There stands the engineer, holding a phone in one hand and waving
//! with the other. You're going so slowly that you haven't even left the
//! station. You exit the gondola.
//!
//! The missing part wasn't the only issue - one of the gears in the engine is
//! wrong. A gear is any `*` symbol that is adjacent to exactly two part
//! numbers. Its gear ratio is the result of multiplying those two numbers
//! together.
//!
//! This time, you need to find the gear ratio of every gear and add them all up
//! so that the engineer can figure out which gear needs to be replaced.
//!
//! Consider the same engine schematic again:
//!
//! ```txt
//! 467..114..
//! ...*......
//! ..35..633.
//! ......#...
//! 617*......
//! .....+.58.
//! ..592.....
//! ......755.
//! ...$.*....
//! .664.598..
//! ```
//!
//! In this schematic, there are two gears. The first is in the top left; it has
//! part numbers 467 and 35, so its gear ratio is 16345. The second gear is in
//! the lower right; its gear ratio is 451490. (The `*` adjacent to 617 is not a
//! gear because it is only adjacent to one part number.) Adding up all of the
//! gear ratios produces 467835.
//!
//! **What is the sum of all of the gear ratios in your engine schematic?**
//!

use std::{path::Path, str::FromStr};

use anyhow::{bail, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Span {
    line: usize,
    start: usize,
    end: usize,
}

impl Span {
    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        let lo_x = if self.start == 0 { 0 } else { self.start - 1 };
        let hi_x = self.end;
        let lo_y = if self.line == 0 { 0 } else { self.line - 1 };
        let hi_y = self.line + 1;

        lo_x <= x && x <= hi_x && lo_y <= y && y <= hi_y
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Gear {
    parts: [u64; 2],
}

impl Gear {
    fn ratio(&self) -> u64 {
        self.parts[0] * self.parts[1]
    }
}

#[derive(Debug)]
struct Schematic {
    grid: Vec<Vec<char>>,
    numbers: Vec<Span>,
    symbols: Vec<(usize, usize)>,
}

impl Schematic {
    fn at(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn span_value(&self, part: &Span) -> Option<u64> {
        let s: String = self.grid[part.line][part.start..part.end].iter().collect();
        u64::from_str(&s).ok()
    }

    fn is_part(&self, span: &Span) -> bool {
        self.symbols.iter().any(|&(x, y)| span.is_adjacent(x, y))
    }

    fn parts(&self) -> Vec<&Span> {
        self.numbers.iter().filter(|&s| self.is_part(s)).collect()
    }

    fn part_numbers(&self) -> Vec<u64> {
        self.parts()
            .into_iter()
            .filter_map(|s| self.span_value(s))
            .collect()
    }

    fn gears(&self) -> Vec<Gear> {
        let mut list = vec![];
        for &(x, y) in &self.symbols {
            if self.at(x, y) != '*' {
                continue;
            }
            let adjacent = self
                .numbers
                .iter()
                .filter(|s| s.is_adjacent(x, y))
                .collect::<Vec<_>>();
            if adjacent.len() == 2 {
                let p1 = self
                    .span_value(adjacent[0])
                    .expect("span should be a part number");
                let p2 = self
                    .span_value(adjacent[1])
                    .expect("span should be a part number");
                list.push(Gear { parts: [p1, p2] });
            }
        }

        list
    }
}

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.is_empty() {
            bail!("grid data must have at least one line")
        }

        let mut grid = vec![];
        let mut parts = vec![];
        let mut symbols = vec![];

        let mut span: Option<Span> = None;
        for (j, line) in s.lines().enumerate() {
            if line.is_empty() {
                bail!("grid line must not be empty");
            }
            grid.push(line.chars().collect());
            let width = line.len();

            for (i, c) in line.char_indices() {
                // track the first digit of the number as we iterate
                if c.is_digit(10) {
                    if span.is_none() {
                        span = Some(Span {
                            line: j,
                            start: i,
                            end: i,
                        });
                    }
                    continue;
                }
                // complete the span if there is one tracked
                if let Some(mut span) = span.take() {
                    span.end = i;
                    parts.push(span);
                }
                // track symbol location regardless of current span
                if c != '.' {
                    symbols.push((i, j));
                }
            }
            // a span can't cross lines, so complete the existing one
            if let Some(mut span) = span.take() {
                span.end = width;
                parts.push(span);
            }
        }

        Ok(Self {
            grid,
            numbers: parts,
            symbols,
        })
    }
}

fn part1(schematic: &Schematic) {
    let sum: u64 = schematic.part_numbers().into_iter().sum();
    println!("Part 1: {sum}");
}

fn part2(schematic: &Schematic) {
    let sum: u64 = schematic.gears().iter().map(Gear::ratio).sum();
    println!("Part 2: {sum}");
}

/// Executes the solution with provided input file.
pub fn exec<P: AsRef<Path>>(path: P) -> Result<()> {
    let schematic = Schematic::from_str(&std::fs::read_to_string(path)?)?;

    part1(&schematic);
    part2(&schematic);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SCHEMATIC_DATA: &str = "\
        467..114.1\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..\n";

    #[test]
    fn parse_schematic() {
        let schematic = Schematic::from_str(SCHEMATIC_DATA).unwrap();
        assert_eq!(schematic.numbers.len(), 11);
        assert_eq!(schematic.symbols.len(), 6);
    }

    #[test]
    fn parse_spans() {
        let nums = vec![467, 114, 1, 35, 633, 617, 58, 592, 755, 664, 598];
        let schematic = Schematic::from_str(SCHEMATIC_DATA).unwrap();

        assert_eq!(schematic.numbers.len(), nums.len());
        for (s, expected) in schematic.numbers.iter().zip(nums) {
            let value = schematic.span_value(s).unwrap();
            assert_eq!(value, expected);
        }
    }

    #[test]
    fn schematic_part_numbers() {
        let nums = vec![467, 35, 633, 617, 592, 755, 664, 598];
        let schematic = Schematic::from_str(SCHEMATIC_DATA).unwrap();
        let part_numbers = schematic.part_numbers();

        assert_eq!(part_numbers.len(), nums.len());
        for (part, expected) in part_numbers.into_iter().zip(nums) {
            assert_eq!(part, expected);
        }
    }

    #[test]
    fn schematic_gears() {
        let nums = vec![Gear { parts: [467, 35] }, Gear { parts: [755, 598] }];
        let schematic = Schematic::from_str(SCHEMATIC_DATA).unwrap();
        let gears = schematic.gears();

        assert_eq!(gears.len(), nums.len());
        for (gear, expected) in gears.into_iter().zip(nums) {
            assert_eq!(gear, expected);
        }
    }
}
