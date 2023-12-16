//! Solution for Advent of Code 2023, Day 9.
//!
//! # Day 9: Mirage Maintenance
//!
//! ## Part One
//!
//! You ride the camel through the sandstorm and stop where the ghost's maps
//! told you to stop. The sandstorm subsequently subsides, somehow seeing you
//! standing at an oasis!
//!
//! The camel goes to get some water and you stretch your neck. As you look up,
//! you discover what must be yet another giant floating island, this one made
//! of metal! That must be where the parts to fix the sand machines come from.
//!
//! There's even a hang glider partially buried in the sand here; once the sun
//! rises and heats up the sand, you might be able to use the glider and the hot
//! air to get all the way up to the metal island!
//!
//! While you wait for the sun to rise, you admire the oasis hidden here in the
//! middle of Desert Island. It must have a delicate ecosystem; you might as
//! well take some ecological readings while you wait. Maybe you can report any
//! environmental instabilities you find to someone so the oasis can be around
//! for the next sandstorm-worn traveler.
//!
//! You pull out your handy Oasis And Sand Instability Sensor and analyze your
//! surroundings. The OASIS produces a report of many values and how they are
//! changing over time (your puzzle input). Each line in the report contains the
//! history of a single value. For example:
//!
//! ```txt
//! 0 3 6 9 12 15
//! 1 3 6 10 15 21
//! 10 13 16 21 30 45
//! ```
//!
//! To best protect the oasis, your environmental report should include a
//! prediction of the next value in each history. To do this, start by making a
//! new sequence from the difference at each step of your history. If that
//! sequence is not all zeroes, repeat this process, using the sequence you just
//! generated as the input sequence. Once all of the values in your latest
//! sequence are zeroes, you can extrapolate what the next value of the original
//! history should be.
//!
//! In the above dataset, the first history is `0 3 6 9 12 15`. Because the
//! values increase by 3 each step, the first sequence of differences that you
//! generate will be `3 3 3 3 3`. Note that this sequence has one fewer value
//! than the input sequence because at each step it considers two numbers from
//! the input. Since these values aren't all zero, repeat the process: the
//! values differ by 0 at each step, so the next sequence is `0 0 0 0`. This
//! means you have enough information to extrapolate the history! Visually,
//! these sequences can be arranged like this:
//!
//! ```txt
//! 0   3   6   9  12  15
//!   3   3   3   3   3
//!     0   0   0   0
//! ```
//!
//! To extrapolate, start by adding a new zero to the end of your list of
//! zeroes; because the zeroes represent differences between the two values
//! above them, this also means there is now a placeholder in every sequence
//! above it:
//!
//! ```txt
//! 0   3   6   9  12  15   B
//!   3   3   3   3   3   A
//!     0   0   0   0   0
//! ```
//!
//! You can then start filling in placeholders from the bottom up. A needs to be
//! the result of increasing 3 (the value to its left) by 0 (the value below
//! it); this means A must be 3:
//!
//! ```txt
//! 0   3   6   9  12  15   B
//!   3   3   3   3   3   3
//!     0   0   0   0   0
//! ```
//!
//! Finally, you can fill in B, which needs to be the result of increasing 15
//! (the value to its left) by 3 (the value below it), or 18:
//!
//! ```txt
//! 0   3   6   9  12  15  18
//!   3   3   3   3   3   3
//!     0   0   0   0   0
//! ```
//!
//! So, the next value of the first history is 18.
//!
//! Finding all-zero differences for the second history requires an additional
//! sequence:
//!
//! ```txt
//! 1   3   6  10  15  21
//!   2   3   4   5   6
//!     1   1   1   1
//!       0   0   0
//! ```
//!
//! Then, following the same process as before, work out the next value in each
//! sequence from the bottom up:
//!
//! ```txt
//! 1   3   6  10  15  21  28
//!   2   3   4   5   6   7
//!     1   1   1   1   1
//!       0   0   0   0
//! ```
//!
//! So, the next value of the second history is 28.
//!
//! The third history requires even more sequences, but its next value can be
//! found the same way:
//!
//! ```txt
//! 10  13  16  21  30  45  68
//!    3   3   5   9  15  23
//!      0   2   4   6   8
//!        2   2   2   2
//!          0   0   0
//! ```
//!
//! So, the next value of the third history is 68.
//!
//! If you find the next value for each history in this example and add them
//! together, you get 114.
//!
//! Analyze your OASIS report and extrapolate the next value for each history.
//! **What is the sum of these extrapolated values?**
//!
//! ## Part Two
//!
//! Of course, it would be nice to have even more history included in your
//! report. Surely it's safe to just extrapolate backwards as well, right?
//!
//! For each history, repeat the process of finding differences until the
//! sequence of differences is entirely zero. Then, rather than adding a zero to
//! the end and filling in the next values of each previous sequence, you should
//! instead add a zero to the beginning of your sequence of zeroes, then fill in
//! new first values for each previous sequence.
//!
//! In particular, here is what the third example history looks like when
//! extrapolating back in time:
//!
//! ```txt
//! 5  10  13  16  21  30  45
//!   5   3   3   5   9  15
//!    -2   0   2   4   6
//!       2   2   2   2
//!         0   0   0
//! ```
//!
//! Adding the new values on the left side of each sequence from bottom to top
//! eventually reveals the new left-most history value: 5.
//!
//! Doing this for the remaining example data above results in previous values
//! of -3 for the first history and 0 for the second history. Adding all three
//! new values together produces 2.
//!
//! Analyze your OASIS report again, this time extrapolating the previous value
//! for each history. **What is the sum of these extrapolated values?**

use std::{collections::VecDeque, path::Path, str::FromStr};

use anyhow::Result;

use crate::fsutils::map_file_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Future,
    Past,
}

#[derive(Debug, PartialEq, Eq)]
struct Reading {
    nums: VecDeque<i64>,
}

impl Reading {
    fn analyze(&self, mode: Mode) -> i64 {
        let mut deque = VecDeque::new();
        deque.push_back(self.nums.clone());
        let mut frame = VecDeque::new();

        while let Some(front) = deque.front() {
            if front.iter().all(|&x| x == 0) {
                break;
            }
            for i in 0..front.len() - 1 {
                frame.push_back(front[i + 1] - front[i]);
            }
            deque.push_front(frame);
            frame = VecDeque::new();
        }

        while deque.len() > 1 {
            frame = deque.pop_front().unwrap();
            let front = deque.front_mut().unwrap();

            match mode {
                Mode::Future => {
                    let i = front.back().unwrap();
                    let j = frame.back().unwrap();
                    front.push_back(i + j);
                }
                Mode::Past => {
                    let i = front.front().unwrap();
                    let j = frame.front().unwrap();
                    front.push_front(i - j);
                }
            };
        }

        let front = deque.front().unwrap();
        match mode {
            Mode::Future => *front.back().unwrap(),
            Mode::Past => *front.front().unwrap(),
        }
    }
}

impl FromStr for Reading {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let nums = s.split(' ').map(i64::from_str).collect::<Result<_, _>>()?;

        Ok(Self { nums })
    }
}

fn part1(readings: &[Reading]) {
    let sum = readings
        .into_iter()
        .fold(0, |value, r| value + r.analyze(Mode::Future));

    println!("Part 1: {sum}");
}

fn part2(readings: &[Reading]) {
    let sum = readings
        .into_iter()
        .fold(0, |value, r| value + r.analyze(Mode::Past));

    println!("Part 2: {sum}");
}

/// Executes the solution with provided input file.
pub fn exec<P: AsRef<Path>>(path: P) -> Result<()> {
    let readings = map_file_lines(path, Reading::from_str)?;

    part1(&readings);
    part2(&readings);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_DATA: &str = "\
        0 3 6 9 12 15\n\
        1 3 6 10 15 21\n\
        10 13 16 21 30 45\n";

    #[test]
    fn parse_reading() {
        let expected = vec![
            Reading {
                nums: VecDeque::from([0, 3, 6, 9, 12, 15]),
            },
            Reading {
                nums: VecDeque::from([1, 3, 6, 10, 15, 21]),
            },
            Reading {
                nums: VecDeque::from([10, 13, 16, 21, 30, 45]),
            },
        ];

        for (s, expected) in EXAMPLE_DATA.lines().zip(expected) {
            let reading = Reading::from_str(s).unwrap();
            assert_eq!(reading, expected);
        }
    }

    #[test]
    fn reading_analyze_future() {
        let expected = vec![18, 28, 68];

        for (s, expected) in EXAMPLE_DATA.lines().zip(expected) {
            let reading = Reading::from_str(s).unwrap();
            assert_eq!(reading.analyze(Mode::Future), expected);
        }
    }

    #[test]
    fn reading_analyze_past() {
        let expected = vec![-3, 0, 5];

        for (s, expected) in EXAMPLE_DATA.lines().zip(expected) {
            let reading = Reading::from_str(s).unwrap();
            assert_eq!(reading.analyze(Mode::Past), expected);
        }
    }
}
