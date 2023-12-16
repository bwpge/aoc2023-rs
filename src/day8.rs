//! Solution for Advent of Code 2023, Day 8.
//!
//! # Day 8: Haunted Wasteland
//!
//! ## Part One
//!
//! You're still riding a camel across Desert Island when you spot a sandstorm
//! quickly approaching. When you turn to warn the Elf, she disappears before
//! your eyes! To be fair, she had just finished warning you about ghosts a few
//! minutes ago.
//!
//! One of the camel's pouches is labeled "maps" - sure enough, it's full of
//! documents (your puzzle input) about how to navigate the desert. At least,
//! you're pretty sure that's what they are; one of the documents contains a
//! list of left/right instructions, and the rest of the documents seem to
//! describe some kind of network of labeled nodes.
//!
//! It seems like you're meant to use the left/right instructions to navigate
//! the network. Perhaps if you have the camel follow the same instructions, you
//! can escape the haunted wasteland!
//!
//! After examining the maps for a bit, two nodes stick out: `AAA` and `ZZZ`.
//! You feel like `AAA` is where you are now, and you have to follow the
//! left/right instructions until you reach `ZZZ`.
//!
//! This format defines each node of the network individually. For example:
//!
//! ```txt
//! RL
//!
//! AAA = (BBB, CCC)
//! BBB = (DDD, EEE)
//! CCC = (ZZZ, GGG)
//! DDD = (DDD, DDD)
//! EEE = (EEE, EEE)
//! GGG = (GGG, GGG)
//! ZZZ = (ZZZ, ZZZ)
//! ```
//!
//! Starting with `AAA`, you need to look up the next element based on the next
//! left/right instruction in your input. In this example, start with `AAA` and go
//! right (`R`) by choosing the right element of AAA, CCC. Then, `L` means to choose
//! the left element of `CCC`, `ZZZ`. By following the left/right instructions, you
//! reach `ZZZ` in 2 steps.
//!
//! Of course, you might not find `ZZZ` right away. If you run out of left/right
//! instructions, repeat the whole sequence of instructions as necessary: RL
//! really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation
//! that takes 6 steps to reach `ZZZ`:
//!
//! ```txt
//! LLR
//!
//! AAA = (BBB, BBB)
//! BBB = (AAA, ZZZ)
//! ZZZ = (ZZZ, ZZZ)
//! ```
//!
//! Starting at `AAA`, follow the left/right instructions. **How many steps are
//! required to reach `ZZZ`?**
//!
//! ## Part Two
//!
//! The sandstorm is upon you and you aren't any closer to escaping the
//! wasteland. You had the camel follow the instructions, but you've barely left
//! your starting position. It's going to take significantly more steps to
//! escape!
//!
//! What if the map isn't for people - what if the map is for ghosts? Are ghosts
//! even bound by the laws of spacetime? Only one way to find out.
//!
//! After examining the maps a bit longer, your attention is drawn to a curious
//! fact: the number of nodes with names ending in A is equal to the number
//! ending in Z! If you were a ghost, you'd probably just start at every node
//! that ends with A and follow all of the paths at the same time until they all
//! simultaneously end up at nodes that end with Z.
//!
//! For example:
//!
//! ```txt
//! LR
//!
//! 11A = (11B, XXX)
//! 11B = (XXX, 11Z)
//! 11Z = (11B, XXX)
//! 22A = (22B, XXX)
//! 22B = (22C, 22C)
//! 22C = (22Z, 22Z)
//! 22Z = (22B, 22B)
//! XXX = (XXX, XXX)
//! ```
//!
//! Here, there are two starting nodes, 11A and 22A (because they both end with
//! A). As you follow each left/right instruction, use that instruction to
//! simultaneously navigate away from both nodes you're currently on. Repeat
//! this process until all of the nodes you're currently on end with Z. (If only
//! some of the nodes you're on end with Z, they act like any other node and you
//! continue as normal.) In this example, you would proceed as follows:
//!
//!   - Step 0: You are at `11A` and `22A`.
//!   - Step 1: You choose all of the left paths, leading you to `11B` and
//!     `22B`.
//!   - Step 2: You choose all of the right paths, leading you to `11Z` and
//!     `22C`.
//!   - Step 3: You choose all of the left paths, leading you to `11B` and
//!     `22Z`.
//!   - Step 4: You choose all of the right paths, leading you to `11Z` and
//!     `22B`.
//!   - Step 5: You choose all of the left paths, leading you to `11B` and
//!     `22C`.
//!   - Step 6: You choose all of the right paths, leading you to `11Z` and
//!     `22Z`.
//!
//! So, in this example, you end up entirely on nodes that end in Z after 6
//! steps.
//!
//! Simultaneously start on every node that ends with A. **How many steps does
//! it take before you're only on nodes that end with Z?**

use std::{collections::HashMap, path::Path, str::FromStr};

use anyhow::{anyhow, Result};
use num::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (name, nodes) = s
            .split_once(" = ")
            .ok_or_else(|| anyhow!("invalid node format"))?;
        let (left, right) = nodes[1..nodes.len() - 1]
            .split_once(", ")
            .ok_or_else(|| anyhow!("invalid node format"))?;

        Ok(Self {
            name: name.into(),
            left: left.into(),
            right: right.into(),
        })
    }
}

#[derive(Debug)]
struct Network {
    map: HashMap<String, Node>,
    instructions: String,
}

impl Network {
    /// Returns the left or right [`Node`] based on given name and instruction.
    fn step(&self, name: &str, instruction: char) -> &Node {
        let node = &self.map[name];
        match instruction {
            'L' => &self.map[&node.left],
            'R' => &self.map[&node.right],
            _ => panic!("unknown instruction"),
        }
    }

    /// Counts the number of steps it takes to move from the starting node until
    /// the predicate `f` returns `true`.
    fn steps<F>(&self, from: &str, f: F) -> u64
    where
        F: Fn(&Node) -> bool,
    {
        assert!(self.map.contains_key(from));

        let mut inst = self.instructions.chars().cycle();
        let mut current = &self.map[from];
        let mut count = 0;

        while !f(current) {
            count += 1;
            current = self.step(&current.name, inst.next().unwrap());
        }

        count
    }

    /// This is a bit of a "meta" solution. The data is structured such that
    /// cycles are formed for each starting `__A` node to an ending `__Z` node.
    /// This isn't really something that would normally check for, but the
    /// problem indicating these are simultaneous steps (with "cycle"
    /// instructions) gives a hint that we need to cycle through and possibly
    /// past the end node.
    ///
    /// Upon inspection of the data, it is even more specialized -- it has the
    /// same number of steps from `__A` to `__Z` as (next after `__Z`) to `__Z`.
    /// This makes the solution fairly simple -- we can just find the number of
    /// steps from `__A` to `__Z` and then get the Least Common Multiple (LCM)
    /// of all the paths to get a final answer.
    fn steps_parallel(&self, from_suffix: &str, to_suffix: &str) -> u64 {
        self.map
            .keys()
            .filter(|&k| k.ends_with(from_suffix))
            .map(|n| self.steps(n, |n| n.name.ends_with(to_suffix)))
            .fold(1, |value, n| n.lcm(&value))
    }
}

impl FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = s.lines();
        let instructions = lines
            .next()
            .ok_or_else(|| anyhow!("file must contain instructions"))?
            .into();
        let map = lines
            .filter_map(|s| Node::from_str(s).ok())
            .map(|n| (n.name.clone(), n))
            .collect::<HashMap<_, _>>();

        Ok(Self { map, instructions })
    }
}

fn part1(network: &Network) {
    println!("Part 1: {}", network.steps("AAA", |n| n.name == "ZZZ"));
}

fn part2(network: &Network) {
    println!("Part 2: {}", network.steps_parallel("A", "Z"));
}

/// Executes the solution with provided input file.
pub fn exec<P: AsRef<Path>>(path: P) -> Result<()> {
    let contents = std::fs::read_to_string(path)?;
    let network = Network::from_str(&contents)?;

    part1(&network);
    part2(&network);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_MAP: &str = "\
        RL\n\
        \n\
        AAA = (BBB, CCC)\n\
        BBB = (DDD, EEE)\n\
        CCC = (ZZZ, GGG)\n\
        DDD = (DDD, DDD)\n\
        EEE = (EEE, EEE)\n\
        GGG = (GGG, GGG)\n\
        ZZZ = (ZZZ, ZZZ)\n";

    #[test]
    fn parse_node() {
        let s = "AAA = (BBB, CCC)";
        let expected = Node {
            name: "AAA".into(),
            left: "BBB".into(),
            right: "CCC".into(),
        };

        let node = Node::from_str(s).unwrap();
        assert_eq!(node, expected);
    }

    #[test]
    fn parse_network() {
        let network = Network::from_str(EXAMPLE_MAP).unwrap();
        assert_eq!(network.instructions, "RL");
        assert_eq!(network.map.len(), 7)
    }

    #[test]
    fn network_steps() {
        let s = "LLR\n\
            \n\
            AAA = (BBB, BBB)\n\
            BBB = (AAA, ZZZ)\n\
            ZZZ = (ZZZ, ZZZ)\n";

        let network = Network::from_str(s).unwrap();
        assert_eq!(network.steps("AAA", |n| n.name == "ZZZ"), 6);
    }

    #[test]
    fn network_steps_parallel() {
        let s = "\
            LR\n\
            \n\
            11A = (11B, XXX)\n\
            11B = (XXX, 11Z)\n\
            11Z = (11B, XXX)\n\
            22A = (22B, XXX)\n\
            22B = (22C, 22C)\n\
            22C = (22Z, 22Z)\n\
            22Z = (22B, 22B)\n\
            XXX = (XXX, XXX)\n";
        let network = Network::from_str(s).unwrap();
        assert_eq!(network.steps_parallel("A", "Z"), 6);
    }
}
