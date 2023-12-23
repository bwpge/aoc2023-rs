use std::{fmt, hash::Hash, str::FromStr};

use crate::{
    dijkstra::{Graph, Vertex},
    Coordinate, Direction, Grid,
};

#[derive(Debug)]
pub struct Block(u8);

impl Block {
    fn weight(&self) -> usize {
        self.0 as usize
    }
}

impl From<char> for Block {
    fn from(value: char) -> Self {
        let v = value.to_digit(10).expect("value should be a digit");
        Self(v as u8)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Encapsulates logic for traversing a [`City`], such as direction tracking.
#[derive(Debug, Clone, Copy, Hash)]
pub struct Node {
    pos: Coordinate,
    dir: Direction,
    count: u8,
}

impl Node {
    fn new(pos: Coordinate, dir: Direction, count: u8) -> Self {
        Self { pos, dir, count }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Node {}

impl Vertex for Node {
    type EqTy = Coordinate;
    type VisitTy = Self;

    fn eq_repr(&self) -> Self::EqTy {
        self.pos
    }

    fn visit_repr(&self) -> Self::VisitTy {
        *self
    }
}

impl From<Coordinate> for Node {
    fn from(value: Coordinate) -> Self {
        Self {
            pos: value,
            dir: Direction::North,
            count: 0,
        }
    }
}

#[derive(Debug)]
pub struct City {
    grid: Grid<Block>,
}

impl City {
    pub const START: Node = Node {
        pos: Coordinate { x: 0, y: 0 },
        dir: Direction::South,
        count: 0,
    };

    /// Returns the most southeast coordinate in the [`City`].
    #[inline]
    pub fn bottom_right(&self) -> Coordinate {
        Coordinate::new(self.grid.width() - 1, self.grid.height() - 1)
    }
}

impl Graph for City {
    type Node = Node;
    type Distance = usize;

    fn adjacent(&self, node: &Self::Node) -> Vec<Self::Node> {
        let mut nodes = vec![];
        for d in Direction::ALL {
            if d == node.dir.opposite() || (node.dir == d && node.count >= 3) {
                continue;
            }
            if let Some(pos) = node.pos.by_direction(d) {
                if !self.grid.contains(pos) {
                    continue;
                }
                let count = 1 + if node.dir == d { node.count } else { 0 };
                nodes.push(Node::new(pos, d, count));
            }
        }

        nodes
    }

    fn edge(&self, _: &Self::Node, to: &Self::Node) -> Self::Distance {
        self.grid[to.pos].weight()
    }
}

impl FromStr for City {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        Ok(Self { grid })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_MAP: &str = "\
        2413432311323\n\
        3215453535623\n\
        3255245654254\n\
        3446585845452\n\
        4546657867536\n\
        1438598798454\n\
        4457876987766\n\
        3637877979653\n\
        4654967986887\n\
        4564679986453\n\
        1224686865563\n\
        2546548887735\n\
        4322674655533\n";

    #[test]
    fn parse_city() {
        let c = City::from_str(EXAMPLE_MAP).unwrap();
        assert_eq!(c.grid.width(), 13);
        assert_eq!(c.grid.height(), 13);
        assert_eq!(c.grid[(12, 0)].0, 3);
        assert_eq!(c.grid[(5, 5)].0, 9);
        assert_eq!(c.grid[(10, 10)].0, 5);
        assert_eq!(c.grid[(0, 12)].0, 4);
        assert_eq!(c.grid[(12, 12)].0, 3);
    }

    #[test]
    fn city_min_distance_example() {
        let c = City::from_str(EXAMPLE_MAP).unwrap();
        let from = City::START;
        let to = c.bottom_right();

        assert_eq!(c.min_distance(from, to), 102);
    }
}
