//! Solution for Advent of Code 2023, Day 7.
//!
//! # Day 7: Camel Cards
//!
//! ## Part One
//!
//! Your all-expenses-paid trip turns out to be a one-way, five-minute ride in
//! an airship. (At least it's a cool airship!) It drops you off at the edge of
//! a vast desert and descends back to Island Island.
//!
//! "Did you bring the parts?"
//!
//! You turn around to see an Elf completely covered in white clothing, wearing
//! goggles, and riding a large camel.
//!
//! "Did you bring the parts?" she asks again, louder this time. You aren't sure
//! what parts she's looking for; you're here to figure out why the sand
//! stopped.
//!
//! "The parts! For the sand, yes! Come with me; I will show you." She beckons
//! you onto the camel.
//!
//! After riding a bit across the sands of Desert Island, you can see what look
//! like very large rocks covering half of the horizon. The Elf explains that
//! the rocks are all along the part of Desert Island that is directly above
//! Island Island, making it hard to even get there. Normally, they use big
//! machines to move the rocks and filter the sand, but the machines have broken
//! down because Desert Island recently stopped receiving the parts they need to
//! fix the machines.
//!
//! You've already assumed it'll be your job to figure out why the parts stopped
//! when she asks if you can help. You agree automatically.
//!
//! Because the journey will take a few days, she offers to teach you the game
//! of Camel Cards. Camel Cards is sort of similar to poker except it's designed
//! to be easier to play while riding a camel.
//!
//! In Camel Cards, you get a list of hands, and your goal is to order them
//! based on the strength of each hand. A hand consists of five cards labeled
//! one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of
//! each card follows this order, where A is the highest and 2 is the lowest.
//!
//! Every hand is exactly one type. From strongest to weakest, they are:
//!
//!   - Five of a kind, where all five cards have the same label: `AAAAA`
//!   - Four of a kind, where four cards have the same label and one card has a
//!     different label: `AA8AA`
//!   - Full house, where three cards have the same label, and the remaining two
//!     cards share a different label: `23332`
//!   - Three of a kind, where three cards have the same label, and the
//!     remaining two cards are each different from any other card in the hand:
//!     `TTT98`
//!   - Two pair, where two cards share one label, two other cards share a
//!     second label, and the remaining card has a third label: `23432`
//!   - One pair, where two cards share one label, and the other three cards
//!     have a different label from the pair and each other: `A23A4`
//!   - High card, where all cards' labels are distinct: `23456`
//!
//! Hands are primarily ordered based on type; for example, every full house is
//! stronger than any three of a kind.
//!
//! If two hands have the same type, a second ordering rule takes effect. Start
//! by comparing the first card in each hand. If these cards are different, the
//! hand with the stronger first card is considered stronger. If the first card
//! in each hand have the same label, however, then move on to considering the
//! second card in each hand. If they differ, the hand with the higher second
//! card wins; otherwise, continue with the third card in each hand, then the
//! fourth, then the fifth.
//!
//! So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger
//! because its first card is stronger. Similarly, 77888 and 77788 are both a
//! full house, but 77888 is stronger because its third card is stronger (and
//! both hands have the same first and second card).
//!
//! To play Camel Cards, you are given a list of hands and their corresponding
//! bid (your puzzle input). For example:
//!
//! ```txt
//! 32T3K 765
//! T55J5 684
//! KK677 28
//! KTJJT 220
//! QQQJA 483
//! ```
//!
//! This example shows five hands; each hand is followed by its bid amount. Each
//! hand wins an amount equal to its bid multiplied by its rank, where the
//! weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up
//! to the strongest hand. Because there are five hands in this example, the
//! strongest hand will have rank 5 and its bid will be multiplied by 5.
//!
//! So, the first step is to put the hands in order of strength:
//!
//!   - `32T3K` is the only one pair and the other hands are all a stronger type,
//!     so it gets rank 1.
//!   - `KK677` and `KTJJT` are both two pair. Their first cards both have the
//!     same label, but the second card of `KK677` is stronger (`K` vs `T`), so
//!     `KTJJT` gets rank 2 and `KK677` gets rank 3.
//!   - `T55J5` and `QQQJA` are both three of a kind. `QQQJA` has a stronger
//!     first card, so it gets rank 5 and `T55J5` gets rank 4.
//!
//! Now, you can determine the total winnings of this set of hands by adding up
//! the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 +
//! 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are 6440.
//!
//! Find the rank of every hand in your set. **What are the total winnings?**
//!
//! ## Part Two
//!
//! To make things a little more interesting, the Elf introduces one additional
//! rule. Now, `J` cards are jokers - wildcards that can act like whatever card
//! would make the hand the strongest type possible.
//!
//! To balance this, `J` cards are now the weakest individual cards, weaker even
//! than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5,
//! 4, 3, 2, J.
//!
//! `J` cards can pretend to be whatever card is best for the purpose of
//! determining hand type; for example, `QJJQ2` is now considered four of a
//! kind. However, for the purpose of breaking ties between two hands of the
//! same type, `J` is always treated as `J`, not the card it's pretending to be:
//! `JKKK2` is weaker than `QQQQ2` because `J` is weaker than `Q`.
//!
//! Now, the above example goes very differently:
//!
//! ```txt
//! 32T3K 765
//! T55J5 684
//! KK677 28
//! KTJJT 220
//! QQQJA 483
//! ```
//!
//!   - `32T3K` is still the only one pair; it doesn't contain any jokers, so
//!     its strength doesn't increase.
//!   - `KK677` is now the only two pair, making it the second-weakest hand.
//!   - `T55J5`, `KTJJT`, and `QQQJA` are now all four of a kind! `T55J5` gets
//!     rank 3, `QQQJA` gets rank 4, and `KTJJT` gets rank 5.
//!
//! With the new joker rule, the total winnings in this example are 5905.
//!
//! Using the new joker rule, find the rank of every hand in your set. **What
//! are the new total winnings?**

use std::{cmp::Ordering, path::Path, str::FromStr};

use anyhow::{anyhow, bail, Result};

use crate::{fsutils::map_file_lines, map};

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuleSet {
    faces: Vec<char>,
    wildcards: Vec<char>,
}

impl RuleSet {
    fn standard() -> Self {
        Self {
            faces: vec![
                '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
            ],
            wildcards: Vec::new(),
        }
    }

    fn jokers_wild() -> Self {
        Self {
            faces: vec![
                'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
            ],
            wildcards: vec!['J'],
        }
    }

    fn is_wild(&self, card: &Card) -> bool {
        if self.wildcards.is_empty() || card.value > self.faces.len() {
            return false;
        }
        self.is_wild_face(self.faces[card.value])
    }

    fn is_wild_face(&self, value: char) -> bool {
        self.wildcards.contains(&value)
    }
}

impl Default for RuleSet {
    fn default() -> Self {
        Self::standard()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card {
    value: usize,
}

impl Card {
    fn with_rules(value: char, rules: &RuleSet) -> Result<Self> {
        Ok(Self {
            value: rules
                .faces
                .iter()
                .position(|c| *c == value)
                .ok_or_else(|| anyhow!("invalid card face `{value}`"))?,
        })
    }
}

impl From<usize> for Card {
    fn from(value: usize) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    rules: RuleSet,
}

impl Hand {
    #[allow(dead_code)]
    fn new(s: &str) -> Result<Self> {
        Self::with_rules(s, &RuleSet::standard())
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

    fn parse(s: &str, rules: &RuleSet) -> Result<Self> {
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

fn winnings(hands: &[Hand]) -> u64 {
    hands.iter().enumerate().fold(0, |value, (i, hand)| {
        let rank = i as u64 + 1;
        value + (hand.bid * rank)
    })
}

fn part1(path: &Path) -> Result<()> {
    let mut hands = map_file_lines(path, |s| Hand::parse(s, &RuleSet::standard()))?;
    hands.sort();

    println!("Part 1: {}", winnings(&hands));

    Ok(())
}

fn part2(path: &Path) -> Result<()> {
    let mut hands = map_file_lines(path, |s| Hand::parse(s, &RuleSet::jokers_wild()))?;
    hands.sort();

    println!("Part 2: {}", winnings(&hands));

    Ok(())
}

/// Executes the solution with provided input file.
pub fn exec<P: AsRef<Path>>(path: P) -> Result<()> {
    part1(path.as_ref())?;
    part2(path.as_ref())?;

    Ok(())
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
