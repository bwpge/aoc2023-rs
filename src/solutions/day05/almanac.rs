use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Result};

use super::seed::{Seed, SeedMode};

trait Mapping {
    fn contains(&self, value: u64) -> bool;

    fn range_contains(&self, value: u64) -> bool;

    fn map(&self, value: u64) -> u64;

    fn invert(&self, value: u64) -> u64;
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Bijection {
    domain: u64,
    range: u64,
    count: u64,
}

impl Mapping for Bijection {
    /// Returns whether or not the domain contains the given `value`.
    fn contains(&self, value: u64) -> bool {
        value >= self.domain && value < (self.domain + self.count)
    }

    /// Returns whether or not the range contains the given `value`.
    fn range_contains(&self, value: u64) -> bool {
        value >= self.range && value < (self.range + self.count)
    }

    /// Maps the `value` from domain to range.
    fn map(&self, value: u64) -> u64 {
        assert!(self.contains(value));
        self.range + (value - self.domain)
    }

    /// Maps the `value` from range to domain.
    fn invert(&self, value: u64) -> u64 {
        assert!(self.range_contains(value));
        self.domain + (value - self.range)
    }
}

impl FromStr for Bijection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let nums = s
            .split(' ')
            .filter(|s| !s.is_empty())
            .filter_map(|s| u64::from_str(s).ok())
            .collect::<Vec<_>>();
        if nums.len() != 3 {
            bail!("bijection must contain exactly 3 numbers");
        }

        Ok(Self {
            domain: nums[1],
            range: nums[0],
            count: nums[2],
        })
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct BijectionList {
    inner: Vec<Bijection>,
}

impl Mapping for BijectionList {
    fn contains(&self, value: u64) -> bool {
        self.inner.iter().any(|b| b.contains(value))
    }

    fn range_contains(&self, value: u64) -> bool {
        self.inner.iter().any(|b| b.range_contains(value))
    }

    fn map(&self, value: u64) -> u64 {
        for b in &self.inner {
            if b.contains(value) {
                return b.map(value);
            }
        }
        value
    }

    fn invert(&self, value: u64) -> u64 {
        for b in &self.inner {
            if b.range_contains(value) {
                return b.invert(value);
            }
        }
        value
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Almanac {
    seeds: Vec<Seed>,
    mappings: HashMap<String, BijectionList>,
}

impl Almanac {
    const MAP_NAMES: [&'static str; 7] = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    fn new() -> Self {
        Self {
            seeds: Default::default(),
            mappings: HashMap::from(Self::MAP_NAMES.map(|s| (s.into(), Default::default()))),
        }
    }

    pub fn seeds(&self) -> &[Seed] {
        &self.seeds
    }

    pub fn parse_with_mode(s: &str, mode: SeedMode) -> Result<Self> {
        let mut result = Self::new();

        let mut section = String::new();
        for line in s.lines().filter(|&s| !s.is_empty()) {
            // parse seeds line
            if line.starts_with("seeds") {
                result.seeds = Seed::parse_list(line, mode)?;
                continue;
            }

            // parse map section name
            if line.ends_with("map:") {
                section = line.strip_suffix(" map:").unwrap_or("").into();
                if !Self::MAP_NAMES.contains(&section.as_str()) {
                    bail!("invalid section name `{section}`");
                }
                continue;
            }

            // parse mapping triplet for current section
            let b = Bijection::from_str(line)?;
            result
                .mappings
                .entry(section.clone())
                .and_modify(|x| x.inner.push(b));
        }

        Ok(result)
    }

    pub fn find_location(&self, seed: u64) -> u64 {
        let mut value = seed;
        for key in Self::MAP_NAMES {
            value = self.mappings[key].map(value);
        }

        value
    }

    pub fn find_seed(&self, location: u64) -> Option<u64> {
        let mut value = location;
        for &key in Self::MAP_NAMES.iter().rev() {
            value = self.mappings[key].invert(value);
        }

        for seed in &self.seeds {
            if seed.contains(value) {
                return Some(value);
            }
        }

        None
    }
}

impl Default for Almanac {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::map;

    use super::*;

    static EXAMPLE_ALMANAC: &str = "\
        seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4\n";

    macro_rules! bijection {
        ($d:literal, $r:literal, $c:literal) => {
            Bijection {
                domain: $d,
                range: $r,
                count: $c,
            }
        };
    }

    fn example_almanac(mode: SeedMode) -> Almanac {
        let mut value = Almanac {
            seeds: vec![],
            mappings: map![
                "seed-to-soil".into() => BijectionList { inner: vec![
                    bijection!(98, 50, 2),
                    bijection!(50, 52, 48),
                ] },
                "soil-to-fertilizer".into() => BijectionList { inner: vec![
                    bijection!(15, 0, 37),
                    bijection!(52, 37, 2),
                    bijection!(0, 39, 15),
                ] },
                "fertilizer-to-water".into() => BijectionList { inner: vec![
                    bijection!(53, 49, 8),
                    bijection!(11, 0, 42),
                    bijection!(0, 42, 7),
                    bijection!(7, 57, 4),
                ] },
                "water-to-light".into() => BijectionList { inner: vec![
                    bijection!(18, 88, 7),
                    bijection!(25, 18, 70),
                ] },
                "light-to-temperature".into() => BijectionList { inner: vec![
                    bijection!(77, 45, 23),
                    bijection!(45, 81, 19),
                    bijection!(64, 68, 13),
                ] },
                "temperature-to-humidity".into() => BijectionList { inner: vec![
                    bijection!(69, 0, 1),
                    bijection!(0, 1, 69),
                ] },
                "humidity-to-location".into() => BijectionList { inner: vec![
                    bijection!(56, 60, 37),
                    bijection!(93, 56, 4),
                ] },
            ],
        };
        if mode == SeedMode::List {
            value.seeds = vec![
                Seed::from(79),
                Seed::from(14),
                Seed::from(55),
                Seed::from(13),
            ];
        } else {
            value.seeds = vec![
                Seed {
                    start: 79,
                    count: 14,
                },
                Seed {
                    start: 55,
                    count: 13,
                },
            ];
        }
        value
    }

    #[test]
    fn bijection_mapping() {
        let b = Bijection {
            domain: 50,
            range: 52,
            count: 48,
        };

        // domain contains
        assert!(!b.contains(49));
        assert!(b.contains(50));
        assert!(b.contains(52));
        assert!(b.contains(79));
        assert!(b.contains(97));
        assert!(!b.contains(98));

        // range contains
        assert!(!b.range_contains(49));
        assert!(!b.range_contains(50));
        assert!(b.range_contains(52));
        assert!(b.range_contains(79));
        assert!(b.range_contains(97));
        assert!(b.range_contains(98));

        // map
        assert_eq!(b.map(50), 52);
        assert_eq!(b.map(97), 99);

        // invert
        assert_eq!(b.invert(98), 96);
        assert_eq!(b.invert(52), 50);
    }

    #[test]
    fn bijection_list_mapping() {
        let b = BijectionList {
            inner: vec![
                Bijection {
                    domain: 98,
                    range: 50,
                    count: 2,
                },
                Bijection {
                    domain: 50,
                    range: 52,
                    count: 48,
                },
            ],
        };

        assert!(b.contains(79));
        assert!(b.contains(98));
        assert!(!b.contains(49));
        assert_eq!(b.map(96), 98);
        assert_eq!(b.map(98), 50);
        assert_eq!(b.map(0), 0);
        assert_eq!(b.map(49), 49);
    }

    #[test]
    fn parse_almanac_seed_list() {
        let mode = SeedMode::List;
        let expected = example_almanac(mode);

        let almanac = Almanac::parse_with_mode(EXAMPLE_ALMANAC, mode).unwrap();
        assert_eq!(almanac, expected);
    }

    #[test]
    fn parse_almanac_seed_range() {
        let expected = vec![
            Seed {
                start: 79,
                count: 14,
            },
            Seed {
                start: 55,
                count: 13,
            },
        ];
        let a = Almanac::parse_with_mode(EXAMPLE_ALMANAC, SeedMode::RangePairs).unwrap();
        assert_eq!(a.seeds, expected)
    }

    #[test]
    fn almanac_find_location() {
        let a = example_almanac(SeedMode::List);

        assert_eq!(a.find_location(79), 82);
        assert_eq!(a.find_location(14), 43);
        assert_eq!(a.find_location(55), 86);
        assert_eq!(a.find_location(13), 35);
    }

    #[test]
    fn almanac_find_seed() {
        let a = example_almanac(SeedMode::List);

        assert_eq!(a.find_seed(82).unwrap(), 79);
        assert_eq!(a.find_seed(43).unwrap(), 14);
        assert_eq!(a.find_seed(86).unwrap(), 55);
        assert_eq!(a.find_seed(35).unwrap(), 13);
        assert_eq!(a.find_seed(0), None);
    }
}
