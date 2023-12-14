//! Solution for Advent of Code 2023, Day 2.
//!
//! # Day 2: Cube Conundrum
//!
//! ## Part One
//!
//! You're launched high into the atmosphere! The apex of your trajectory just
//! barely reaches the surface of a large island floating in the sky. You gently
//! land in a fluffy pile of leaves. It's quite cold, but you don't see much
//! snow. An Elf runs over to greet you.
//!
//! The Elf explains that you've arrived at Snow Island and apologizes for the
//! lack of snow. He'll be happy to explain the situation, but it's a bit of a
//! walk, so you have some time. They don't get many visitors up here; would you
//! like to play a game in the meantime?
//!
//! As you walk, the Elf shows you a small bag and some cubes which are either
//! red, green, or blue. Each time you play this game, he will hide a secret
//! number of cubes of each color in the bag, and your goal is to figure out
//! information about the number of cubes.
//!
//! To get information, once a bag has been loaded with cubes, the Elf will
//! reach into the bag, grab a handful of random cubes, show them to you, and
//! then put them back in the bag. He'll do this a few times per game.
//!
//! You play several games and record the information from each game (your
//! puzzle input). Each game is listed with its ID number (like the 11 in `Game
//! 11: ...`) followed by a semicolon-separated list of subsets of cubes that
//! were revealed from the bag (like 3 red, 5 green, 4 blue).
//!
//! For example, the record of a few games might look like this:
//!
//! ```txt
//! Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//! Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
//! Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
//! Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
//! Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
//! ```
//!
//! In game 1, three sets of cubes are revealed from the bag (and then put back
//! again). The first set is 3 blue cubes and 4 red cubes; the second set is 1
//! red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green
//! cubes.
//!
//! The Elf would first like to know which games would have been possible if the
//! bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
//!
//! In the example above, games 1, 2, and 5 would have been possible if the bag
//! had been loaded with that configuration. However, game 3 would have been
//! impossible because at one point the Elf showed you 20 red cubes at once;
//! similarly, game 4 would also have been impossible because the Elf showed you
//! 15 blue cubes at once. If you add up the IDs of the games that would have
//! been possible, you get 8.
//!
//! Determine which games would have been possible if the bag had been loaded
//! with only 12 red cubes, 13 green cubes, and 14 blue cubes. **What is the sum
//! of the IDs of those games?**
//!
//! ## Part Two
//!
//! The Elf says they've stopped producing snow because they aren't getting any
//! water! He isn't sure why the water stopped; however, he can show you how to
//! get to the water source to check it out for yourself. It's just up ahead!
//!
//! As you continue your walk, the Elf poses a second question: in each game you
//! played, what is the fewest number of cubes of each color that could have
//! been in the bag to make the game possible?
//!
//! Again consider the example games from earlier:
//!
//! ```txt
//! Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//! Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
//! Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
//! Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
//! Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
//! ```
//!
//!   - In game 1, the game could have been played with as few as 4 red, 2
//!   green, and 6 blue cubes. If any color had even one fewer cube, the game
//!   would have been impossible.
//!   - Game 2 could have been played with a minimum of 1 red, 3 green, and 4
//!   blue cubes.
//!   - Game 3 must have been played with at least 20 red, 13 green, and 6 blue
//!   cubes.
//!   - Game 4 required at least 14 red, 3 green, and 15 blue cubes.
//!   - Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
//!
//! The power of a set of cubes is equal to the numbers of red, green, and blue
//! cubes multiplied together. The power of the minimum set of cubes in game 1
//! is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up
//! these five powers produces the sum 2286.
//!
//! For each game, find the minimum set of cubes that must have been present.
//! **What is the sum of the power of these sets?**
//!

use std::{path::Path, str::FromStr};

use anyhow::{bail, Context as _, Result};

use crate::fsutils::map_file_lines;

/// A sample of revealed cubes.
#[derive(Debug, Default, PartialEq, Eq)]
struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}

impl Sample {
    fn power(&self) -> u64 {
        u64::from(self.red) * u64::from(self.green) * u64::from(self.blue)
    }
}

impl FromStr for Sample {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut sample = Self::default();
        let splits = s.split(", ").into_iter().map(|item| {
            item.split_once(' ')
                .expect("number and color name should be separated by a space")
        });
        for (n, color) in splits {
            let num = n.parse::<u32>()?;
            match color {
                "red" => sample.red = num,
                "green" => sample.green = num,
                "blue" => sample.blue = num,
                _ => bail!("unknown color `{}`", color),
            };
        }

        Ok(sample)
    }
}

/// A single round of the cube game, with various samples of revealed cubes .
#[derive(Debug, Default, PartialEq, Eq)]
struct Game {
    id: u32,
    samples: Vec<Sample>,
}

impl Game {
    fn is_possible(&self, sample: &Sample) -> bool {
        self.samples
            .iter()
            .all(|s| sample.red >= s.red && sample.green >= s.green && sample.blue >= s.blue)
    }

    fn power(&self) -> u64 {
        let mut min_sample = Sample::default();
        for sample in &self.samples {
            min_sample.red = min_sample.red.max(sample.red);
            min_sample.green = min_sample.green.max(sample.green);
            min_sample.blue = min_sample.blue.max(sample.blue);
        }

        min_sample.power()
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (prefix, samplestr) = s.split_once(": ").context("game line is not valid")?;
        let (_, game) = prefix.split_once(' ').context("invalid game prefix")?;
        let id = game.parse::<u32>()?;

        let mut samples: Vec<Sample> = Vec::new();
        for sample in samplestr.split("; ") {
            samples.push(Sample::from_str(sample)?);
        }

        Ok(Game { id, samples })
    }
}

fn part1<'g, It>(games: It)
where
    It: Iterator<Item = &'g Game>,
{
    let sample = Sample {
        red: 12,
        green: 13,
        blue: 14,
    };

    let sum: u32 = games
        .filter(|&g| g.is_possible(&sample))
        .map(|g| g.id)
        .sum();
    println!("Part 1: {sum}");
}

fn part2<'g, It>(games: It)
where
    It: Iterator<Item = &'g Game>,
{
    let sum: u64 = games.map(Game::power).sum();
    println!("Part 2: {sum}");
}

/// Executes the solution with provided input file.
pub fn exec<P: AsRef<Path>>(path: P) -> Result<()> {
    let games = map_file_lines(path, Game::from_str)?;

    part1(games.iter());
    part2(games.iter());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_GAMES: [&str; 5] = [
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    #[test]
    fn parse_sample() {
        let lines = vec!["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"];
        let expect_list = vec![
            Sample {
                red: 4,
                green: 0,
                blue: 3,
            },
            Sample {
                red: 1,
                green: 2,
                blue: 6,
            },
            Sample {
                red: 0,
                green: 2,
                blue: 0,
            },
        ];

        for (&line, expected) in lines.iter().zip(expect_list) {
            let game = Sample::from_str(line).unwrap();
            assert_eq!(game, expected);
        }
    }

    #[test]
    fn sample_power() {
        let sample = Sample {
            red: 1,
            green: 2,
            blue: 6,
        };
        assert_eq!(sample.power(), 12);
    }

    #[test]
    fn parse_games() {
        let lines = vec!["Game 42: 15 blue", "Game 100: 2 green; 1 red"];
        let expect_list = vec![
            Game {
                id: 42,
                samples: vec![Sample {
                    red: 0,
                    green: 0,
                    blue: 15,
                }],
            },
            Game {
                id: 100,
                samples: vec![
                    Sample {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                    Sample {
                        red: 1,
                        green: 0,
                        blue: 0,
                    },
                ],
            },
        ];

        for (&line, expected) in lines.iter().zip(expect_list) {
            let game = Game::from_str(line).unwrap();
            assert_eq!(game, expected);
        }
    }

    #[test]
    fn example_games_possible() {
        let possible = vec![true, true, false, false, true];
        let sample = Sample {
            red: 12,
            green: 13,
            blue: 14,
        };

        for (&s, expected) in EXAMPLE_GAMES.iter().zip(possible) {
            let game = Game::from_str(s).unwrap();
            assert_eq!(game.is_possible(&sample), expected);
        }
    }

    #[test]
    fn example_game_power() {
        let powers = vec![48, 12, 1560, 630, 36];

        for (&s, expected) in EXAMPLE_GAMES.iter().zip(powers) {
            let game = Game::from_str(s).unwrap();
            assert_eq!(game.power(), expected, "with game = {:?}", game);
        }
    }
}
