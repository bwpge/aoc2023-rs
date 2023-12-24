use std::{fmt, hash::Hash, marker::PhantomData, str::FromStr};

use crate::{dijkstra::Graph, Coordinate, Direction, Grid};

#[derive(Debug, Clone, Copy)]
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

/// A type that implements additional logic for Dijkstra's algorithm in
/// [`Graph`].
pub trait DijkstraExt: Sized {
    /// Returns a list of nodes Athis type considers adjacent, given a current
    /// `node` and a `city`.
    fn adjacent(node: &Node, city: &City<Self>) -> Vec<Node>;

    /// Returns whether or not this type considers the algorithm finished, given
    /// a current `node`.
    fn is_done(_node: &Node) -> bool {
        true
    }
}

/// Encapsulates logic for traversing a [`City`] with Dijkstra's algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl From<Coordinate> for Node {
    fn from(value: Coordinate) -> Self {
        Self {
            pos: value,
            dir: Direction::North,
            count: 0,
        }
    }
}

/// Marker type that implements regular crucible traversal logic for a [`City`].
pub struct Crucible;

impl DijkstraExt for Crucible {
    fn adjacent(node: &Node, city: &City<Self>) -> Vec<Node> {
        // movement restrictions:
        // - maximum of 3 in same direction
        // - can't move backwards

        let mut nodes = vec![];
        for d in Direction::ALL {
            if d == node.dir.opposite() || (node.dir == d && node.count >= 3) {
                continue;
            }
            if let Some(pos) = node.pos.by_direction(d) {
                if !city.grid.contains(pos) {
                    continue;
                }
                let count = 1 + if node.dir == d { node.count } else { 0 };
                nodes.push(Node::new(pos, d, count));
            }
        }

        nodes
    }
}

/// Marker type that implements ultra crucible traversal logic for a [`City`].
pub struct UltraCrucible;

impl DijkstraExt for UltraCrucible {
    fn adjacent(node: &Node, city: &City<Self>) -> Vec<Node> {
        // movement restrictions:
        // - minimum of 4 in same direction
        // - maximum of 10 in same direction
        // - can't move backwards
        // NOTE: this implementation needs to account for the starting node
        // which will have a direction with a count of 0; it shouldn't be forced
        // to continue in the starting direction. all other nodes will have at
        // least 1 movement.

        let mut nodes = vec![];
        for d in Direction::ALL {
            if d == node.dir.opposite()
                || (node.dir == d && node.count >= 10)
                || (node.dir != d && node.count < 4 && node.count > 0)
            {
                continue;
            }
            if let Some(pos) = node.pos.by_direction(d) {
                if !city.grid.contains(pos) {
                    continue;
                }
                let count = 1 + if node.dir == d { node.count } else { 0 };
                nodes.push(Node::new(pos, d, count));
            }
        }

        nodes
    }

    fn is_done(node: &Node) -> bool {
        // this drove me crazy until i learned how to read:
        // > Once an ultra crucible starts moving in a direction, it needs to
        // > move a minimum of four blocks in that direction before it can turn
        // > **(or even before it can stop at the end)**
        // thus, the search cannot be considered complete if it has not moved at
        // least 4 times in the current direction
        node.count >= 4
    }
}

/// A map of the city blocks, containing heat loss information in each cell.
#[derive(Debug)]
pub struct City<T: DijkstraExt> {
    grid: Grid<Block>,
    marker: PhantomData<T>,
}

impl<T: DijkstraExt> City<T> {
    const START: Node = Node {
        pos: Coordinate { x: 0, y: 0 },
        dir: Direction::South,
        count: 0,
    };

    /// Traverses the city from top-left to bottom-right, following pathing
    /// rules of this city's crucible.
    pub fn traverse(&self) -> usize {
        self.min_distance(Self::START, self.bottom_right())
    }

    /// Consumes this [`City`] and changes the crucible marker type.
    pub fn set_crucible<C: DijkstraExt>(self) -> City<C> {
        City {
            grid: self.grid,
            marker: PhantomData,
        }
    }

    /// Returns the most southeast coordinate in the [`City`].
    #[inline]
    fn bottom_right(&self) -> Coordinate {
        Coordinate::new(self.grid.width() - 1, self.grid.height() - 1)
    }
}

impl<T: DijkstraExt> Graph for City<T> {
    type Node = Node;
    type Distance = usize;

    fn adjacent(&self, node: &Self::Node) -> Vec<Self::Node> {
        T::adjacent(node, self)
    }

    fn edge(&self, _: &Self::Node, to: &Self::Node) -> Self::Distance {
        self.grid[to.pos].weight()
    }

    fn nodes_eq(lhs: &Self::Node, rhs: &Self::Node) -> bool {
        // the algorithm can consider a node match simply based on coordinates
        lhs.pos == rhs.pos
    }

    fn is_done(&self, current: &Self::Node, _: &Self::Node) -> bool {
        // each crucible has different logic for when the search is actually
        // finished, even if destination coordinates are a match
        T::is_done(current)
    }
}

impl<T: DijkstraExt> FromStr for City<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        Ok(Self {
            grid,
            marker: PhantomData,
        })
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

    static EXAMPLE_MAP_ULTRA: &str = "\
        111111111111\n\
        999999999991\n\
        999999999991\n\
        999999999991\n\
        999999999991\n";

    macro_rules! city {
        () => {
            City::<Crucible>::from_str(EXAMPLE_MAP).unwrap()
        };
        ($val:expr) => {
            City::<Crucible>::from_str($val).unwrap()
        };
        ($ty:ty, $val:expr) => {
            City::<$ty>::from_str($val).unwrap()
        };
    }

    #[test]
    fn parse_city() {
        let c = city!();
        assert_eq!(c.grid.width(), 13);
        assert_eq!(c.grid.height(), 13);
        assert_eq!(c.grid[(12, 0)].0, 3);
        assert_eq!(c.grid[(5, 5)].0, 9);
        assert_eq!(c.grid[(10, 10)].0, 5);
        assert_eq!(c.grid[(0, 12)].0, 4);
        assert_eq!(c.grid[(12, 12)].0, 3);
    }

    #[test]
    fn city_example_traverse() {
        let c = city!();
        assert_eq!(c.traverse(), 102);
    }

    #[test]
    fn city_traverse_ultra() {
        let c = city!(UltraCrucible, EXAMPLE_MAP_ULTRA);
        assert_eq!(c.traverse(), 71);
    }

    #[test]
    fn city_example_traverse_ultra() {
        let c = city!(UltraCrucible, EXAMPLE_MAP);
        assert_eq!(c.traverse(), 94);
    }
}
