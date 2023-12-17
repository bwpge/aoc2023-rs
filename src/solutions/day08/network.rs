use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;
use num::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    pub fn name(&self) -> &str {
        &self.name
    }
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
pub struct Network {
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
    pub fn steps<F>(&self, from: &str, f: F) -> u64
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
    pub fn steps_parallel(&self, from_suffix: &str, to_suffix: &str) -> u64 {
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
