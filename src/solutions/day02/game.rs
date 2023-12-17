use std::str::FromStr;

use anyhow::{bail, Context};

/// A sample of revealed cubes.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Sample {
    red: u32,
    green: u32,
    blue: u32,
}

impl Sample {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    pub fn power(&self) -> u64 {
        u64::from(self.red) * u64::from(self.green) * u64::from(self.blue)
    }
}

impl FromStr for Sample {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut sample = Self::default();
        let splits = s.split(", ").map(|item| {
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
pub struct Game {
    id: u32,
    samples: Vec<Sample>,
}

impl Game {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_possible(&self, sample: &Sample) -> bool {
        self.samples
            .iter()
            .all(|s| sample.red >= s.red && sample.green >= s.green && sample.blue >= s.blue)
    }

    pub fn power(&self) -> u64 {
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
