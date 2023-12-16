//! Solution for Advent of Code 2023, Day 6.
//!
//! # Day 6: Wait For It
//!
//! ## Part One
//!
//! The ferry quickly brings you across Island Island. After asking around, you
//! discover that there is indeed normally a large pile of sand somewhere near
//! here, but you don't see anything besides lots of water and the small island
//! where the ferry has docked.
//!
//! As you try to figure out what to do next, you notice a poster on a wall near
//! the ferry dock. "Boat races! Open to the public! Grand prize is an
//! all-expenses-paid trip to Desert Island!" That must be where the sand comes
//! from! Best of all, the boat races are starting in just a few minutes.
//!
//! You manage to sign up as a competitor in the boat races just in time. The
//! organizer explains that it's not really a traditional race - instead, you
//! will get a fixed amount of time during which your boat has to travel as far
//! as it can, and you win if your boat goes the farthest.
//!
//! As part of signing up, you get a sheet of paper (your puzzle input) that
//! lists the time allowed for each race and also the best distance ever
//! recorded in that race. To guarantee you win the grand prize, you need to
//! make sure you go farther in each race than the current record holder.
//!
//! The organizer brings you over to the area where the boat races are held. The
//! boats are much smaller than you expected - they're actually toy boats, each
//! with a big button on top. Holding down the button charges the boat, and
//! releasing the button allows the boat to move. Boats move faster if their
//! button was held longer, but time spent holding the button counts against the
//! total race time. You can only hold the button at the start of the race, and
//! boats don't move until the button is released.
//!
//! For example:
//!
//! ```txt
//! Time:      7  15   30
//! Distance:  9  40  200
//! ```
//!
//! This document describes three races:
//!
//!   - The first race lasts 7 milliseconds. The record distance in this race is
//!     9 millimeters.
//!   - The second race lasts 15 milliseconds. The record
//!     distance in this race is 40 millimeters.
//!   - The third race lasts 30 milliseconds. The record distance in this race
//!     is 200 millimeters.
//!
//! Your toy boat has a starting speed of zero millimeters per millisecond. For
//! each whole millisecond you spend at the beginning of the race holding down
//! the button, the boat's speed increases by one millimeter per millisecond.
//!
//! So, because the first race lasts 7 milliseconds, you only have a few options:
//!
//!   - Don't hold the button at all (that is, hold it for 0 milliseconds) at
//!     the start of the race. The boat won't move; it will have traveled 0
//!     millimeters by the end of the race.
//!   - Hold the button for 1 millisecond at the start of the race. Then, the
//!     boat will travel at a speed of 1 millimeter per millisecond for 6
//!     milliseconds, reaching a total distance traveled of 6 millimeters.
//!   - Hold the button for 2 milliseconds, giving the boat a speed of 2
//!     millimeters per millisecond. It will then get 5 milliseconds to move,
//!     reaching a total distance of 10 millimeters.
//!   - Hold the button for 3 milliseconds. After its remaining 4 milliseconds
//!     of travel time, the boat will have gone 12 millimeters.
//!   - Hold the button for 4 milliseconds. After its remaining 3 milliseconds
//!     of travel time, the boat will have gone 12 millimeters.
//!   - Hold the button for 5 milliseconds, causing the boat to travel a total
//!     of 10 millimeters.
//!   - Hold the button for 6 milliseconds, causing the boat to travel a total
//!     of 6 millimeters.
//!   - Hold the button for 7 milliseconds. That's the entire duration of the
//!     race. You never let go of the button. The boat can't move until you let
//!     go of the button. Please make sure you let go of the button so the boat
//!     gets to move. 0 millimeters.
//!
//! Since the current record for this race is 9 millimeters, there are actually
//! 4 different ways you could win: you could hold the button for 2, 3, 4, or 5
//! milliseconds at the start of the race.
//!
//! In the second race, you could hold the button for at least 4 milliseconds
//! and at most 11 milliseconds and beat the record, a total of 8 different ways
//! to win.
//!
//! In the third race, you could hold the button for at least 11 milliseconds
//! and no more than 19 milliseconds and still beat the record, a total of 9
//! ways you could win.
//!
//! To see how much margin of error you have, determine the number of ways you
//! can beat the record in each race; in this example, if you multiply these
//! values together, you get 288 (4 * 8 * 9).
//!
//! Determine the number of ways you could beat the record in each race. **What
//! do you get if you multiply these numbers together?**
//!
//! ## Part Two
//!
//! As the race is about to start, you realize the piece of paper with race
//! times and record distances you got earlier actually just has very bad
//! kerning. There's really only one race - ignore the spaces between the
//! numbers on each line.
//!
//! So, the example from before:
//!
//! ```txt
//! Time:      7  15   30
//! Distance:  9  40  200
//! ```
//!
//! ...now instead means this:
//!
//! ```txt
//! Time:      71530
//! Distance:  940200
//! ```
//!
//! Now, you have to figure out how many ways there are to win this single race.
//! In this example, the race lasts for 71530 milliseconds and the record
//! distance you need to beat is 940200 millimeters. You could hold the button
//! anywhere from 14 to 71516 milliseconds and beat the record, a total of 71503
//! ways!
//!
//! **How many ways can you beat the record in this one much longer race?**
//!

use std::{path::Path, str::FromStr};

use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RaceFormat {
    Multiple,
    Single,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn parse_format(s: &str, format: RaceFormat) -> Result<Vec<Self>> {
        let mut times = None;
        let mut distances = None;

        for line in s.lines() {
            if !line.contains(':') {
                bail!("incorrect race data format");
            }
            let (prefix, numstr) = line
                .split_once(':')
                .expect("line should contain a ':' separator");

            let nums = match format {
                RaceFormat::Multiple => numstr
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(u64::from_str)
                    .collect::<Result<Vec<_>, _>>()?,
                RaceFormat::Single => vec![u64::from_str(numstr.replace(' ', "").as_str())?],
            };
            match prefix {
                "Time" => times = Some(nums),
                "Distance" => distances = Some(nums),
                _ => bail!("unknown data prefix `{}`", prefix),
            };
        }
        if times.is_none() || distances.is_none() {
            bail!("missing required data");
        }
        let t = times.expect("times should be some");
        let d = distances.expect("distances should be some");
        assert!(!t.is_empty() && t.len() == d.len());

        let result = match format {
            RaceFormat::Multiple => t.into_iter().zip(d).map(|(t, d)| Race::new(t, d)).collect(),
            RaceFormat::Single => vec![Race::new(t[0], d[0])],
        };

        Ok(result)
    }

    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn can_win(&self, charge_ms: f64) -> bool {
        let t = self.time as f64;
        if charge_ms >= t {
            return false;
        }

        // v * t > d
        charge_ms * (t - charge_ms) > (self.distance as f64)
    }

    fn win_condition(&self) -> (u64, u64) {
        // ensure quadratic solution will be real numbers
        assert!((self.time * self.time) >= (4 * self.distance));

        let t = self.time as f64;
        let d = self.distance as f64;
        let s = ((t * t) - (4. * d)).sqrt();
        let mut t_lo = ((t - s) / 2.).round();
        let mut t_hi = ((t + s) / 2.).round();

        // fix rounding logic
        if !self.can_win(t_lo) {
            t_lo += 1.;
        }
        if !self.can_win(t_hi) {
            t_hi -= 1.;
        }

        assert!(t_lo >= 0. && t_hi >= 0. && t_hi >= t_lo);
        assert!(self.can_win(t_lo) && self.can_win(t_hi));

        (t_lo as u64, t_hi as u64)
    }

    fn margin(&self) -> u64 {
        let (t_lo, t_hi) = self.win_condition();

        t_hi + 1 - t_lo
    }
}

fn part1(s: &str) {
    let races = Race::parse_format(s, RaceFormat::Multiple).expect("race data should be valid");
    let margin = races.iter().fold(1, |value, race| value * race.margin());

    println!("Part 1: {}", margin);
}

fn part2(s: &str) {
    let race = Race::parse_format(s, RaceFormat::Single)
        .expect("race data should be valid")
        .into_iter()
        .next()
        .unwrap();

    println!("Part 2: {}", race.margin());
}

/// Executes the solution with provided input file.
pub fn exec<P: AsRef<Path>>(path: P) -> Result<()> {
    let contents = std::fs::read_to_string(path)?;

    part1(&contents);
    part2(&contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_RACES: &str = "\
        Time:      7  15   30\n\
        Distance:  9  40  200\n";

    #[test]
    fn parse_race_multi() {
        let expected = vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)];
        let races = Race::parse_format(EXAMPLE_RACES, RaceFormat::Multiple).unwrap();

        assert_eq!(races.len(), expected.len());
        for (race, expected) in races.into_iter().zip(expected) {
            assert_eq!(race, expected);
        }
    }

    #[test]
    fn parse_race_single() {
        let races = Race::parse_format(EXAMPLE_RACES, RaceFormat::Single).unwrap();
        let expected = Race::new(71530, 940200);

        assert_eq!(races.len(), 1);
        assert_eq!(races[0], expected);
    }

    #[test]
    fn race_can_win() {
        let race = Race::new(7, 9);

        for i in 0..2 {
            assert!(!race.can_win(i as f64));
        }
        for i in 2..6 {
            assert!(race.can_win(i as f64));
        }
        for i in 6..10 {
            assert!(!race.can_win(i as f64));
        }
    }

    #[test]
    fn race_win_conditions() {
        let test_data = vec![
            (Race::new(7, 9), 2, 5),
            (Race::new(15, 40), 4, 11),
            (Race::new(30, 200), 11, 19),
        ];

        for (race, expected_lo, expected_hi) in test_data {
            let (t_lo, t_hi) = race.win_condition();
            assert_eq!(t_lo, expected_lo);
            assert_eq!(t_hi, expected_hi);
        }
    }

    #[test]
    fn race_margins() {
        let test_data = vec![
            (Race::new(7, 9), 4),
            (Race::new(15, 40), 8),
            (Race::new(30, 200), 9),
        ];

        for (race, expected) in test_data {
            assert_eq!(race.margin(), expected);
        }
    }
}
