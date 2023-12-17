use std::{collections::HashSet, str::FromStr};

use anyhow::anyhow;

pub struct Card {
    id: u64,
    numbers: Vec<u64>,
    winning: HashSet<u64>,
}

impl Card {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn matches(&self) -> u64 {
        self.numbers
            .iter()
            .filter(|&n| self.winning.contains(n))
            .count() as u64
    }

    pub fn points(&self) -> u64 {
        let count = self.matches();
        if count == 0 {
            0
        } else {
            1 << (count - 1)
        }
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (prefix, body) = s.split_once(':').expect("line should have a `:` separator");
        let id = prefix
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid prefix format"))
            .map(|(_, id)| u64::from_str(id.trim()))??;
        let (winningstr, numstr) = body
            .split_once('|')
            .ok_or_else(|| anyhow!("invalid prefix format"))?;
        let winning = winningstr
            .split(' ')
            .filter(|&s| !s.is_empty())
            .filter_map(|s| u64::from_str(s).ok())
            .collect();
        let numbers = numstr
            .split(' ')
            .filter(|&s| !s.is_empty())
            .filter_map(|s| u64::from_str(s).ok())
            .collect();

        Ok(Self {
            id,
            numbers,
            winning,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::set;

    use super::*;

    static EXAMPLE_CARDS: &str = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n";

    #[test]
    fn parse_cards() {
        let numbers = vec![
            vec![83, 86, 6, 31, 17, 9, 48, 53],
            vec![61, 30, 68, 82, 17, 32, 24, 19],
            vec![69, 82, 63, 72, 16, 21, 14, 1],
            vec![59, 84, 76, 51, 58, 5, 54, 83],
            vec![88, 30, 70, 12, 93, 22, 82, 36],
            vec![74, 77, 10, 23, 35, 67, 36, 11],
        ];
        let winning: Vec<HashSet<u64>> = vec![
            set![41, 48, 83, 86, 17],
            set![13, 32, 20, 16, 61],
            set![1, 21, 53, 59, 44],
            set![41, 92, 73, 84, 69],
            set![87, 83, 26, 28, 32],
            set![31, 18, 13, 56, 72],
        ];
        let cards = EXAMPLE_CARDS
            .lines()
            .map(|s| Card::from_str(s).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(cards.len(), 6);
        for (card, nums) in cards.iter().zip(numbers) {
            assert_eq!(card.numbers.len(), nums.len());
            for (&i, j) in card.numbers.iter().zip(nums) {
                assert_eq!(i, j)
            }
        }

        for (card, expected) in cards.iter().zip(winning) {
            assert_eq!(card.winning.len(), expected.len());
            for value in expected {
                assert!(card.winning.contains(&value))
            }
        }
    }

    #[test]
    fn card_matches() {
        let matches = [4, 2, 2, 1, 0, 0];
        let cards = EXAMPLE_CARDS
            .lines()
            .map(|s| Card::from_str(s).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(cards.len(), matches.len());
        for (card, expected) in cards.iter().zip(matches) {
            assert_eq!(card.matches(), expected);
        }
    }

    #[test]
    fn card_points() {
        let points = [8, 2, 2, 1, 0, 0];
        let cards = EXAMPLE_CARDS
            .lines()
            .map(|s| Card::from_str(s).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(cards.len(), points.len());
        for (card, expected) in cards.iter().zip(points) {
            assert_eq!(card.points(), expected);
        }
    }
}
