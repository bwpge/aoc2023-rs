use std::{cmp::Ordering, str::FromStr};

use anyhow::{anyhow, bail, Result};

use crate::map;

use super::card::{Card, RuleSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug)]
pub struct Hand {
    cards: [Card; 5],
    bid: u64,
    rules: RuleSet,
}

impl Hand {
    #[allow(dead_code)]
    fn new(s: &str) -> Result<Self> {
        Self::with_rules(s, &RuleSet::standard())
    }

    pub fn bid(&self) -> u64 {
        self.bid
    }

    fn with_rules(s: &str, rules: &RuleSet) -> Result<Self> {
        if s.len() != 5 {
            bail!("hand must have exactly 5 cards");
        }
        let cards = s
            .chars()
            .map(|c| Card::with_rules(c, rules))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            cards: cards.try_into().expect("card vector should be valid array"),
            bid: 0,
            rules: rules.clone(),
        })
    }

    pub fn parse(s: &str, rules: &RuleSet) -> Result<Self> {
        let (cards, bid) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid hand format"))?;

        let mut hand = Self::with_rules(cards, rules)?;
        hand.bid = u64::from_str(bid)?;

        Ok(hand)
    }

    fn kind(&self) -> HandKind {
        // get a count of each card type in the hand
        let mut counts = map![];
        let mut wilds = 0;
        for card in &self.cards {
            if self.rules.is_wild(card) {
                wilds += 1;
            } else {
                counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
            }
        }

        // should not be possible, but account for a hand of all wildcards
        if wilds >= 5 {
            return HandKind::FiveKind;
        }

        // collect and sort values highest to lowest. note, it doesn't matter
        // what the cards actually are, just their counts
        let mut values = counts.values().copied().collect::<Vec<_>>();
        values.sort_by(|a, b| b.cmp(a));

        // since this game does not have flushes or straights (e.g., hand value
        // from sequential cards), jokers can only provide value by matching a
        // card in the hand. it then follows that they can only provide the best
        // value by matching the top counted card. by adding the wildcard count
        // to highest (0-th) value, we can pick the category like any other hand
        assert!(!values.is_empty());
        values[0] += wilds;

        // the highest counted value can quickly indicate what type of hand we have
        match values[0] {
            5 => HandKind::FiveKind,
            4 => HandKind::FourKind,
            3 => {
                match values[1] {
                    2 => HandKind::FullHouse, // 3 - 2
                    _ => HandKind::ThreeKind, // 3 - 1 - 1
                }
            }
            2 => {
                match values[1] {
                    2 => HandKind::TwoPair, // 2 - 2 - 1
                    _ => HandKind::OnePair, // 2 - 1 - 1 - 1
                }
            }
            _ => HandKind::HighCard,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.rules == other.rules
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // hand kind takes priority
        let k1 = self.kind();
        let k2 = other.kind();
        if k1 != k2 {
            return k1.cmp(&k2);
        }

        // otherwise, check the first different card
        for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
            if c1 != c2 {
                return c1.cmp(c2);
            }
        }

        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_parse() {
        let s = "T55J5 684";
        let expected = Hand {
            cards: [
                Card::from(8),
                Card::from(3),
                Card::from(3),
                Card::from(9), // J
                Card::from(3),
            ],
            bid: 684,
            rules: RuleSet::standard(),
        };
        let hand = Hand::parse(s, &RuleSet::standard()).unwrap();

        assert_eq!(hand, expected);
    }

    #[test]
    fn hand_parse_wild() {
        let s = "T55J5 684";
        let expected = Hand {
            cards: [
                Card::from(9),
                Card::from(4),
                Card::from(4),
                Card::from(0), // J
                Card::from(4),
            ],
            bid: 684,
            rules: RuleSet::jokers_wild(),
        };
        let hand = Hand::parse(s, &RuleSet::jokers_wild()).unwrap();

        assert_eq!(hand, expected);
    }

    #[test]
    fn hand_kind() {
        let test_data = vec![
            ("23456", HandKind::HighCard),
            ("KK677", HandKind::TwoPair),
            ("KTJJT", HandKind::TwoPair),
            ("QQQJA", HandKind::ThreeKind),
            ("23332", HandKind::FullHouse),
            ("AA8AA", HandKind::FourKind),
            ("AAAAA", HandKind::FiveKind),
        ];

        for (s, kind) in test_data {
            let hand = Hand::new(s).unwrap();
            assert_eq!(hand.kind(), kind);
        }
    }

    #[test]
    fn hand_compare() {
        let high_card = Hand::new("23456").unwrap();
        let two_pair1 = Hand::new("KK677").unwrap();
        let two_pair2 = Hand::new("KTJJT").unwrap();
        let three_kind = Hand::new("QQQJA").unwrap();
        let full_house = Hand::new("23332").unwrap();
        let four_kind = Hand::new("AA8AA").unwrap();
        let aces = Hand::new("AAAAA").unwrap();

        // same kind, but two_pair2 has a lower second card than two_pair1
        assert_eq!(two_pair1.kind(), two_pair2.kind());
        assert!(two_pair2 < two_pair1);

        // aces should beat all kinds
        assert!(high_card < aces);
        assert!(two_pair1 < aces);
        assert!(two_pair2 < aces);
        assert!(three_kind < aces);
        assert!(full_house < aces);
        assert!(four_kind < aces);

        // high card should lose to all other kinds
        assert!(high_card < two_pair1);
        assert!(high_card < two_pair2);
        assert!(high_card < three_kind);
        assert!(high_card < full_house);
        assert!(high_card < four_kind);
        assert!(high_card < aces);
    }

    #[test]
    fn hand_sort() {
        let expected = vec![
            Hand::new("32T3K").unwrap(),
            Hand::new("KTJJT").unwrap(),
            Hand::new("KK677").unwrap(),
            Hand::new("T55J5").unwrap(),
            Hand::new("QQQJA").unwrap(),
        ];
        let mut hands = vec![
            Hand::new("32T3K").unwrap(),
            Hand::new("T55J5").unwrap(),
            Hand::new("KK677").unwrap(),
            Hand::new("KTJJT").unwrap(),
            Hand::new("QQQJA").unwrap(),
        ];

        assert_ne!(hands, expected);
        hands.sort();
        assert_eq!(hands, expected);
    }

    #[test]
    fn hand_sort_wild() {
        let expected = vec![
            Hand::with_rules("32T3K", &RuleSet::jokers_wild()).unwrap(),
            Hand::with_rules("KK677", &RuleSet::jokers_wild()).unwrap(),
            Hand::with_rules("T55J5", &RuleSet::jokers_wild()).unwrap(),
            Hand::with_rules("QQQJA", &RuleSet::jokers_wild()).unwrap(),
            Hand::with_rules("KTJJT", &RuleSet::jokers_wild()).unwrap(),
        ];
        let mut hands = vec![
            Hand::with_rules("32T3K", &RuleSet::jokers_wild()).unwrap(),
            Hand::with_rules("T55J5", &RuleSet::jokers_wild()).unwrap(),
            Hand::with_rules("KK677", &RuleSet::jokers_wild()).unwrap(),
            Hand::with_rules("KTJJT", &RuleSet::jokers_wild()).unwrap(),
            Hand::with_rules("QQQJA", &RuleSet::jokers_wild()).unwrap(),
        ];

        assert_ne!(hands, expected);
        hands.sort();
        assert_eq!(hands, expected);
    }
}
