use std::str::FromStr;

use anyhow::{anyhow, bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeedMode {
    List,
    RangePairs,
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Seed {
    pub start: u64,
    pub count: u64,
}

impl Seed {
    /// Parses a line pf seed numbers.
    ///
    /// The `mode` will determine if each number is treated as a separate seed,
    /// or as a pair of ranges.
    pub fn parse_list(s: &str, mode: SeedMode) -> Result<Vec<Self>> {
        if !s.contains(':') {
            bail!("invalid seed list format");
        }
        let nums = s
            .split_once(':')
            .ok_or_else(|| anyhow!("invalid list format"))?
            .1
            .split(' ')
            .filter_map(|s| u64::from_str(s).ok())
            .collect::<Vec<_>>();

        if mode == SeedMode::List {
            return Ok(nums.into_iter().map(Self::from).collect());
        }
        if nums.len() % 2 != 0 {
            bail!("seed range list must have even number of items");
        }

        Ok(nums.as_slice().chunks_exact(2).map(Self::from).collect())
    }

    pub fn contains(&self, value: u64) -> bool {
        value >= self.start && value < (self.start + self.count)
    }
}

impl From<u64> for Seed {
    fn from(value: u64) -> Self {
        Self {
            start: value,
            count: 1,
        }
    }
}

impl From<(u64, u64)> for Seed {
    fn from(value: (u64, u64)) -> Self {
        Self {
            start: value.0,
            count: value.1,
        }
    }
}

impl From<&[u64]> for Seed {
    fn from(value: &[u64]) -> Self {
        assert!(value.len() == 2);
        Self::from((value[0], value[1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_parse_list() {
        let nums = vec![79, 14, 55, 13];
        let seeds = Seed::parse_list("seeds: 79 14 55 13", SeedMode::List).unwrap();

        for (seed, expected) in seeds.iter().zip(nums) {
            assert_eq!(seed.start, expected);
            assert_eq!(seed.count, 1);
        }
    }

    #[test]
    fn seed_parse_range_list() {
        let values = vec![(79, 14), (55, 13)];
        let seeds = Seed::parse_list("seeds: 79 14 55 13", SeedMode::RangePairs).unwrap();

        for (seed, (expect_start, expect_count)) in seeds.iter().zip(values) {
            assert_eq!(seed.start, expect_start);
            assert_eq!(seed.count, expect_count);
        }
    }

    #[test]
    fn seed_contains() {
        let seed = Seed::from(42);
        assert!(!seed.contains(41));
        assert!(seed.contains(42));
        assert!(!seed.contains(43));

        let seed = Seed::from((42, 12));
        assert!(!seed.contains(41));
        assert!(seed.contains(42));
        assert!(seed.contains(43));
        assert!(seed.contains(53));
        assert!(!seed.contains(54));
    }
}
