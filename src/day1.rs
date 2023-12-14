//! Solution for Advent of Code 2023, Day 1.
//!
//! # Day 1: Trebuchet?!
//!
//! ## Part One
//!
//! Something is wrong with global snow production, and you've been selected to
//! take a look. The Elves have even given you a map; on it, they've used stars
//! to mark the top fifty locations that are likely to be having problems.
//!
//! You've been doing this long enough to know that to restore snow operations,
//! you need to check all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each
//! day in the Advent calendar; the second puzzle is unlocked when you complete
//! the first. Each puzzle grants one star. Good luck!
//!
//! You try to ask why they can't just use a weather machine ("not powerful
//! enough") and where they're even sending you ("the sky") and why your map
//! looks mostly blank ("you sure ask a lot of questions") and hang on did you
//! just say the sky ("of course, where do you think snow comes from") when you
//! realize that the Elves are already loading you into a trebuchet ("please
//! hold still, we need to strap you in").
//!
//! As they're making the final adjustments, they discover that their calibration
//! document (your puzzle input) has been amended by a very young Elf who was
//! apparently just excited to show off her art skills. Consequently, the Elves
//! are having trouble reading the values on the document.
//!
//! The newly-improved calibration document consists of lines of text; each line
//! originally contained a specific calibration value that the Elves now need to
//! recover. On each line, the calibration value can be found by combining the
//! first digit and the last digit (in that order) to form a single two-digit
//! number.
//!
//! For example:
//!
//! ```txt
//! 1abc2
//! pqr3stu8vwx
//! a1b2c3d4e5f
//! treb7uchet
//! ```
//!
//! In this example, the calibration values of these four lines are 12, 38, 15,
//! and 77. Adding these together produces 142.
//!
//! Consider your entire calibration document. **What is the sum of all of the
//! calibration values?**
//!
//! ## Part Two
//!
//! Your calculation isn't quite right. It looks like some of the digits are
//! actually spelled out with letters: one, two, three, four, five, six, seven,
//! eight, and nine also count as valid "digits".
//!
//! Equipped with this new information, you now need to find the real first and
//! last digit on each line. For example:
//!
//! ```txt
//!  two1nine
//!  eightwothree
//!  abcone2threexyz
//!  xtwone3four
//!  4nineeightseven2
//!  zoneight234
//!  7pqrstsixteen
//! ```
//!
//! In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
//! Adding these together produces 281.
//!
//! **What is the sum of all of the calibration values?**
//!

use std::{path::Path, str::Lines};

use anyhow::Result;

static SPELLED_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Style {
    DigitOnly,
    DigitOrWord,
}

fn decode<It>(value: &str, it: It, style: Style) -> u32
where
    It: Iterator<Item = (usize, char)>,
{
    for (i, c) in it {
        if let Some(num) = c.to_digit(10) {
            return num;
        }
        if style == Style::DigitOrWord {
            for (num, &s) in SPELLED_DIGITS.iter().enumerate() {
                if value[i..].starts_with(s) {
                    return num as u32;
                }
            }
        }
    }

    unreachable!()
}

fn sum_decoded(lines: Lines<'_>, style: Style) -> u32 {
    let mut sum = 0;
    for line in lines {
        let hi = decode(line, line.char_indices(), style);
        let lo = decode(line, line.char_indices().rev(), style);
        sum += (hi * 10) + lo;
    }

    return sum;
}

fn part1(lines: Lines<'_>) {
    println!("Part 1: {}", sum_decoded(lines, Style::DigitOnly));
}

fn part2(lines: Lines<'_>) {
    println!("Part 2: {}", sum_decoded(lines, Style::DigitOrWord));
}

/// Executes the solution with provided input file.
pub fn exec<P: AsRef<Path>>(path: P) -> Result<()> {
    let contents = std::fs::read_to_string(path).expect("file should be valid");

    part1(contents.lines());
    part2(contents.lines());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decde_first_digit_only() {
        let values = vec![
            ("1abc2", 1),
            ("pqr3stu8vwx", 3),
            ("a1b2c3d4e5f", 1),
            ("treb7uchet", 7),
        ];

        for (input, expected) in values {
            let result = decode(input, input.char_indices(), Style::DigitOnly);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn decde_last_digit_only() {
        let values = vec![
            ("1abc2", 2),
            ("pqr3stu8vwx", 8),
            ("a1b2c3d4e5f", 5),
            ("treb7uchet", 7),
        ];

        for (input, expected) in values {
            let result = decode(input, input.char_indices().rev(), Style::DigitOnly);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn decde_first_with_words() {
        let values = vec![
            ("two1nine", 2),
            ("eightwothree", 8),
            ("abcone2threexyz", 1),
            ("xtwone3four", 2),
            ("4nineeightseven2", 4),
            ("zoneight234", 1),
            ("7pqrstsixteen", 7),
        ];

        for (input, expected) in values {
            let result = decode(input, input.char_indices(), Style::DigitOrWord);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn decde_last_with_words() {
        let values = vec![
            ("two1nine", 9),
            ("eightwothree", 3),
            ("abcone2threexyz", 3),
            ("xtwone3four", 4),
            ("4nineeightseven2", 2),
            ("zoneight234", 4),
            ("7pqrstsixteen", 6),
        ];

        for (input, expected) in values {
            let result = decode(input, input.char_indices().rev(), Style::DigitOrWord);
            assert_eq!(result, expected);
        }
    }
}
