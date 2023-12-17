use std::str::FromStr;

use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Multiple,
    Single,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn parse_format(s: &str, format: Format) -> Result<Vec<Self>> {
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
                Format::Multiple => numstr
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(u64::from_str)
                    .collect::<Result<Vec<_>, _>>()?,
                Format::Single => vec![u64::from_str(numstr.replace(' ', "").as_str())?],
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
            Format::Multiple => t.into_iter().zip(d).map(|(t, d)| Race::new(t, d)).collect(),
            Format::Single => vec![Race::new(t[0], d[0])],
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

    pub fn margin(&self) -> u64 {
        let (t_lo, t_hi) = self.win_condition();

        t_hi + 1 - t_lo
    }
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
        let races = Race::parse_format(EXAMPLE_RACES, Format::Multiple).unwrap();

        assert_eq!(races.len(), expected.len());
        for (race, expected) in races.into_iter().zip(expected) {
            assert_eq!(race, expected);
        }
    }

    #[test]
    fn parse_race_single() {
        let races = Race::parse_format(EXAMPLE_RACES, Format::Single).unwrap();
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
