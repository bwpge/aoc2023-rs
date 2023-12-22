use std::{collections::HashSet, str::FromStr};

use crate::{Coordinate, Direction, Grid};

/// Represents a tile in the cave grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    /// An empty space.
    Empty,
    /// A mirror that reflects along north/east and south/west directions.
    MirrorForward,
    /// A mirror that reflects along north/west and south/east directions.
    MirrorBack,
    /// A horizontal splitter.
    SplitterH,
    /// A vertical splitter.
    SplitterV,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Tile::*;

        match value {
            '.' => Empty,
            '/' => MirrorForward,
            '\\' => MirrorBack,
            '-' => SplitterH,
            '|' => SplitterV,
            _ => panic!("unknown tile `{value}`"),
        }
    }
}

/// A beam of light in the cave.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Beam {
    pos: Coordinate,
    direction: Direction,
}

impl Beam {
    /// Creates a new [`Beam`] with position and facing direction.
    pub fn new<C: Into<Coordinate>>(pos: C, direction: Direction) -> Self {
        Self {
            pos: pos.into(),
            direction,
        }
    }
}

/// A contraption consisting of mirrors and splitters that focus a [`Beam`] to
/// energize tiles in the cave.
pub struct Map {
    grid: Grid<Tile>,
}

impl Map {
    /// The default starting beam (top-left, heading right).
    pub const STARTING_BEAM: Beam = Beam {
        pos: Coordinate { x: 0, y: 0 },
        direction: Direction::East,
    };

    /// Traces a beam of light through the contraption using DFS and returns the
    /// number of energized tiles.
    ///
    /// # Panics
    ///
    /// Panics if the starting [`Beam`] position is out of bounds.
    pub fn trace(&self, start: Beam) -> usize {
        assert!(self.grid.contains(start.pos));

        let mut beams = Vec::from([start]);
        // we need to store the entire beam with position/direction since beams
        // can cause cycles. we also need to let beams overlap if they are
        // moving in different directions over the same tile.
        let mut visited = HashSet::new();

        while let Some(b) = beams.pop() {
            // IMPORTANT: check grid contains first to short circuit
            if !self.grid.contains(b.pos) || !visited.insert(b.clone()) {
                continue;
            }

            beams.extend(self.energize(b).into_iter().filter_map(|b| self.advance(b)));
        }

        // we need to only count coordinates for result
        HashSet::<Coordinate>::from_iter(visited.into_iter().map(|b| b.pos)).len()
    }

    /// Traces every beam of light from the edge of the map facing inward and
    /// returns the maximum number of tiles that can be energized.
    ///
    /// For example, a beam of light starting on the south edge will face north,
    /// right edge will face west, etc.
    ///
    /// This is a purely brute force solution and does not use any kind of
    /// memoization or caching.
    pub fn trace_max(&self) -> usize {
        let mut count = 0;

        let y_max = self.grid.height() - 1;
        for x in 0..self.grid.width() {
            count = count.max(self.trace(Beam::new((x, 0), Direction::South)));
            count = count.max(self.trace(Beam::new((x, y_max), Direction::North)));
        }
        let x_max = self.grid.width() - 1;
        for y in 0..self.grid.height() {
            count = count.max(self.trace(Beam::new((1, y), Direction::East)));
            count = count.max(self.trace(Beam::new((x_max, y), Direction::East)));
        }

        count
    }

    /// Moves the beam one tile in it's current facing direction.
    ///
    /// Returns [`None`] if the next position is invalid or out of bounds.
    #[must_use]
    fn advance(&self, b: Beam) -> Option<Beam> {
        b.pos
            .by_direction(b.direction)
            .map(|pos| Beam::new(pos, b.direction))
            .filter(|b| self.grid.contains(b.pos))
    }

    /// Returns one or more [`Beam`] objects after processing splitters or
    /// mirrors at the current position.
    ///
    /// Note that this method **does not** check the boundaries of the input
    /// beam `b` position. The caller is expected to use an input with a valid
    /// grid position.
    fn energize(&self, b: Beam) -> Vec<Beam> {
        use Tile::*;

        let mut beams = vec![];
        let tile = self.grid[b.pos];
        match tile {
            Empty => beams.push(b),
            MirrorForward | MirrorBack => beams.push(self.reflect(b, tile)),
            SplitterH | SplitterV => beams.extend(self.split(b, tile)),
        };

        beams
    }

    /// Handles mirror logic at the input [`Beam`] position.
    fn reflect(&self, mut b: Beam, tile: Tile) -> Beam {
        use Direction::*;
        use Tile::*;

        b.direction = match tile {
            MirrorForward => match b.direction {
                North => East,
                East => North,
                South => West,
                West => South,
            },
            MirrorBack => match b.direction {
                North => West,
                East => South,
                South => East,
                West => North,
            },
            _ => unreachable!(),
        };

        b
    }

    /// Handles splitter logic at the input [`Beam`] position.
    ///
    /// If the beam is split, the position is not changed (it is duplicated with
    /// split directions).
    fn split(&self, b: Beam, tile: Tile) -> Vec<Beam> {
        use Direction::*;
        use Tile::*;

        match (tile, b.direction) {
            // beam is parallel, remains unchanged
            (SplitterH, East | West) | (SplitterV, North | South) => vec![b],
            // beam is perpendicular (horizontal)
            (SplitterH, North | South) => {
                vec![Beam::new(b.pos, East), Beam::new(b.pos, West)]
            }
            // beam is perpendicular (vertical)
            (SplitterV, East | West) => {
                vec![Beam::new(b.pos, North), Beam::new(b.pos, South)]
            }
            _ => unreachable!(),
        }
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;

        Ok(Self { grid })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_MAP: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn parse_map() {
        let m = Map::from_str(EXAMPLE_MAP).unwrap();
        assert_eq!(m.grid.width(), 10);
        assert_eq!(m.grid.height(), 10);
    }

    #[test]
    fn map_trace() {
        let m = Map::from_str(EXAMPLE_MAP).unwrap();
        assert_eq!(m.trace(Map::STARTING_BEAM), 46);
    }

    #[test]
    fn map_trace_max() {
        let m = Map::from_str(EXAMPLE_MAP).unwrap();
        assert_eq!(m.trace_max(), 51);
    }
}
