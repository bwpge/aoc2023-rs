use std::{
    collections::HashMap,
    fmt::{self, Write},
    str::FromStr,
};

use anyhow::{anyhow, bail};

use crate::map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Status {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => bail!("unknown spring status `{value}`"),
        }
    }
}

/// A spring condition record.
#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    springs: Vec<Status>,
    counts: Vec<usize>,
}

impl Record {
    /// Counts the total number of valid arrangements for this record.
    pub fn arrangements(&self) -> u64 {
        Self::arrangements_impl(&self.springs, &self.counts, &mut map![])
    }

    /// Counts the total number of valid arrangements for this record when
    /// unfolded `count` times.
    pub fn arrangements_unfold(&self, count: usize) -> u64 {
        if count <= 1 {
            return self.arrangements();
        }

        let mut springs = self.springs.clone();
        springs.push(Status::Unknown);
        springs = springs.repeat(count);
        springs.pop();
        let counts = self.counts.repeat(count);

        Self::arrangements_impl(&springs, &counts, &mut map![])
    }

    /// This implementation was adapted from [ropewalker]'s solution. The
    /// general strategy is to step through each column of the status and
    /// calculate different arrangements for the current sliding window.
    ///
    /// A cache is used to store known configurations and speed up calculations
    /// for future iterations. A tuple of `(springs, counts)` is used for a
    /// unique key -- this is important because certain configurations might
    /// be the same but for different stages in the `counts` phase.
    ///
    /// [ropewalker]: https://github.com/ropewalker/advent_of_code_2023/blob/e3146fe35ec96684ee9004cd5896bc1d7cc38faa/src/day12.rs
    fn arrangements_impl(
        springs: &[Status],
        counts: &[usize],
        cache: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        let key = (springs.len(), counts.len());

        if let Some(&value) = cache.get(&key) {
            return value;
        }
        if counts.is_empty() {
            let value = u64::from(!springs.contains(&Status::Damaged));
            cache.insert(key, value);
            return value;
        }

        let mut count = 0;
        for i in 0..springs.len() {
            let j = i + counts[0];
            let next = j + 1;

            if springs[0..i].contains(&Status::Damaged) || j > springs.len() {
                break;
            }
            if springs[i..j].contains(&Status::Operational) {
                continue;
            }

            if counts.len() == 1 {
                if j == springs.len() {
                    count += 1;
                    break;
                } else {
                    count += Self::arrangements_impl(&springs[j..], &[], cache);
                    continue;
                };
            } else if springs.len() <= next {
                break;
            } else if springs[j] == Status::Damaged {
                continue;
            }

            count += Self::arrangements_impl(&springs[next..], &counts[1..], cache);
        }

        cache.insert(key, count);

        count
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for s in &self.springs {
            match s {
                Status::Operational => f.write_char('.'),
                Status::Damaged => f.write_char('#'),
                Status::Unknown => f.write_char('?'),
            }?;
        }
        write!(
            f,
            " {}",
            self.counts
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, counts) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid spring record"))?;
        let springs = springs
            .chars()
            .map(Status::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let counts = counts
            .split(',')
            .map(usize::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { springs, counts })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_DATA: &str = "\
        ???.### 1,1,3\n\
        .??..??...?##. 1,1,3\n\
        ?#?#?#?#?#?#?#? 1,3,1,6\n\
        ????.#...#... 4,1,1\n\
        ????.######..#####. 1,6,5\n\
        ?###???????? 3,2,1\n";

    #[test]
    fn record_parse() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        let record = Record::from_str(input).unwrap();

        assert_eq!(record.springs.len(), 15);
        assert_eq!(record.counts.len(), 4);
    }

    #[test]
    fn record_arrangements() {
        let expected = vec![1, 4, 1, 1, 4, 10];
        let records = EXAMPLE_DATA
            .lines()
            .map(|line| Record::from_str(line).unwrap())
            .collect::<Vec<_>>();

        let mut sum = 0;
        for (record, expected) in records.into_iter().zip(expected) {
            let count = record.arrangements();
            assert_eq!(count, expected, "with record: {record}");
            sum += count;
        }
        assert_eq!(sum, 21);
    }

    #[test]
    fn record_arrangements_unfolded() {
        let record = Record::from_str("?###???????? 3,2,1").unwrap();
        assert_eq!(record.arrangements_unfold(5), 506250);
    }
}
