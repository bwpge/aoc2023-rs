//! Solution for Advent of Code 2023, Day 5.
//!
//! # Day 5: If You Give A Seed A Fertilizer
//!
//! ## Part One
//!
//! You take the boat and find the gardener right where you were told he would
//! be: managing a giant "garden" that looks more to you like a farm.
//!
//! "A water source? Island Island is the water source!" You point out that Snow
//! Island isn't receiving any water.
//!
//! "Oh, we had to stop the water because we ran out of sand to filter it with!
//! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand
//! soon; we only turned off the water a few days... weeks... oh no." His face
//! sinks into a look of horrified realization.
//!
//! "I've been so busy making sure everyone here has food that I completely
//! forgot to check why we stopped getting more sand! There's a ferry leaving
//! soon that is headed over in that direction - it's much faster than your
//! boat. Could you please go check it out?"
//!
//! You barely have time to agree to this request when he brings up another.
//! "While you wait for the ferry, maybe you can help us with our food
//! production problem. The latest Island Island Almanac just arrived and we're
//! having trouble making sense of it."
//!
//! The almanac (your puzzle input) lists all of the seeds that need to be
//! planted. It also lists what type of soil to use with each kind of seed, what
//! type of fertilizer to use with each kind of soil, what type of water to use
//! with each kind of fertilizer, and so on. Every type of seed, soil,
//! fertilizer and so on is identified with a number, but numbers are reused by
//! each category - that is, soil 123 and fertilizer 123 aren't necessarily
//! related to each other.
//!
//! For example:
//!
//! ```txt
//! seeds: 79 14 55 13
//!
//! seed-to-soil map:
//! 50 98 2
//! 52 50 48
//!
//! soil-to-fertilizer map:
//! 0 15 37
//! 37 52 2
//! 39 0 15
//!
//! fertilizer-to-water map:
//! 49 53 8
//! 0 11 42
//! 42 0 7
//! 57 7 4
//!
//! water-to-light map:
//! 88 18 7
//! 18 25 70
//!
//! light-to-temperature map:
//! 45 77 23
//! 81 45 19
//! 68 64 13
//!
//! temperature-to-humidity map:
//! 0 69 1
//! 1 0 69
//!
//! humidity-to-location map:
//! 60 56 37
//! 56 93 4
//! ```
//!
//! The almanac starts by listing which seeds need to be planted: seeds 79, 14,
//! 55, and 13.
//!
//! The rest of the almanac contains a list of maps which describe how to
//! convert numbers from a source category into numbers in a destination
//! category. That is, the section that starts with seed-to-soil map: describes
//! how to convert a seed number (the source) to a soil number (the
//! destination). This lets the gardener and his team know which soil to use
//! with which seeds, which water to use with which fertilizer, and so on.
//!
//! Rather than list every source number and its corresponding destination
//! number one by one, the maps describe entire ranges of numbers that can be
//! converted. Each line within a map contains three numbers: the destination
//! range start, the source range start, and the range length.
//!
//! Consider again the example seed-to-soil map:
//!
//! ```txt
//! 50 98 2
//! 52 50 48
//! ```
//!
//! The first line has a destination range start of 50, a source range start of
//! 98, and a range length of 2. This line means that the source range starts at
//! 98 and contains two values: 98 and 99. The destination range is the same
//! length, but it starts at 50, so its two values are 50 and 51. With this
//! information, you know that seed number 98 corresponds to soil number 50 and
//! that seed number 99 corresponds to soil number 51.
//!
//! The second line means that the source range starts at 50 and contains 48
//! values: 50, 51, ..., 96, 97. This corresponds to a destination range
//! starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed
//! number 53 corresponds to soil number 55.
//!
//! Any source numbers that aren't mapped correspond to the same destination
//! number. So, seed number 10 corresponds to soil number 10.
//!
//! So, the entire list of seed numbers and their corresponding soil numbers
//! looks like this:
//!
//! ```txt
//! seed  soil
//! 0     0
//! 1     1
//! ...   ...
//! 48    48
//! 49    49
//! 50    52
//! 51    53
//! ...   ...
//! 96    98
//! 97    99
//! 98    50
//! 99    51
//! ```
//!
//! With this map, you can look up the soil number required for each initial
//! seed number:
//!
//!   - Seed number 79 corresponds to soil number 81.
//!   - Seed number 14 corresponds to soil number 14.
//!   - Seed number 55 corresponds to soil number 57.
//!   - Seed number 13 corresponds to soil number 13.
//!
//! The gardener and his team want to get started as soon as possible, so they'd
//! like to know the closest location that needs a seed. Using these maps, find
//! the lowest location number that corresponds to any of the initial seeds. To
//! do this, you'll need to convert each seed number through other categories
//! until you can find its corresponding location number. In this example, the
//! corresponding types are:
//!
//!   - Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78,
//!     humidity 78, location 82.
//!   - Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42,
//!     humidity 43, location 43.
//!   - Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82,
//!     humidity 82, location 86.
//!   - Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34,
//!     humidity 35, location 35.
//!
//! So, the lowest location number in this example is 35.
//!
//! **What is the lowest location number that corresponds to any of the initial
//! seed numbers?**
//!
//! ## Part Two
//!
//! Everyone will starve if you only plant such a small number of seeds.
//! Re-reading the almanac, it looks like the seeds: line actually describes
//! ranges of seed numbers.
//!
//! The values on the initial seeds: line come in pairs. Within each pair, the
//! first value is the start of the range and the second value is the length of
//! the range. So, in the first line of the example above:
//!
//! ```txt
//! seeds: 79 14 55 13
//! ```
//!
//! This line describes two ranges of seed numbers to be planted in the garden.
//! The first range starts with seed number 79 and contains 14 values: 79, 80,
//! ..., 91, 92. The second range starts with seed number 55 and contains 13
//! values: 55, 56, ..., 66, 67.
//!
//! Now, rather than considering four seed numbers, you need to consider a total
//! of 27 seed numbers.
//!
//! In the above example, the lowest location number can be obtained from seed
//! number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77,
//! temperature 45, humidity 46, and location 46. So, the lowest location number
//! is 46.
//!
//! Consider all of the initial seed numbers listed in the ranges on the first
//! line of the almanac. **What is the lowest location number that corresponds
//! to any of the initial seed numbers?**
//!

use std::{collections::HashMap, path::Path, str::FromStr};

use anyhow::{anyhow, bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SeedMode {
    List,
    RangePairs,
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct Seed {
    start: u64,
    count: u64,
}

impl Seed {
    /// Parses a line pf seed numbers.
    ///
    /// The `mode` will determine if each number is treated as a separate seed,
    /// or as a pair of ranges.
    fn parse_list(s: &str, mode: SeedMode) -> Result<Vec<Self>> {
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

    fn contains(&self, value: u64) -> bool {
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

trait Mapping {
    fn contains(&self, value: u64) -> bool;

    fn range_contains(&self, value: u64) -> bool;

    fn map(&self, value: u64) -> u64;

    fn invert(&self, value: u64) -> u64;
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Bijection {
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
struct Almanac {
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

    fn parse_with_mode(s: &str, mode: SeedMode) -> Result<Self> {
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

    fn find_location(&self, seed: u64) -> u64 {
        let mut value = seed;
        for key in Self::MAP_NAMES {
            value = *&self.mappings[key].map(value);
        }

        value
    }

    fn find_seed(&self, location: u64) -> Option<u64> {
        let mut value = location;
        for &key in Self::MAP_NAMES.iter().rev() {
            value = *&self.mappings[key].invert(value);
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

fn part1(almanac: Almanac) {
    let mut locs = vec![];
    for seed in &almanac.seeds {
        locs.push(almanac.find_location(seed.start));
    }
    let value = locs.iter().min().expect("locations should not be empty");

    println!("Part 1: {}", value);
}

fn part2(almanac: Almanac) {
    let mut seed = None;

    let mut idx = 0;
    while seed.is_none() {
        seed = almanac.find_seed(idx);
        idx += 1;
    }

    println!("Part 2: {}", idx - 1);
}

/// Executes the solution with provided input file.
pub fn exec<P: AsRef<Path>>(_path: P) -> Result<()> {
    let contents = std::fs::read_to_string(_path)?;

    part1(Almanac::parse_with_mode(contents.as_str(), SeedMode::List)?);
    part2(Almanac::parse_with_mode(
        contents.as_str(),
        SeedMode::RangePairs,
    )?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::map;

    use super::*;

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
        let data = include_str!("../data/day5-example.txt");

        let almanac = Almanac::parse_with_mode(data, mode).unwrap();
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
        let a = Almanac::parse_with_mode(
            include_str!("../data/day5-example.txt"),
            SeedMode::RangePairs,
        )
        .unwrap();
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
