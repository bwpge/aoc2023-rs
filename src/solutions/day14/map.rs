use std::{
    collections::HashMap,
    fmt::{self, Write},
    str::FromStr,
};

use crate::{map, Coordinate, Direction, Grid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Cube,
    Rounded,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty => f.write_char('.'),
            Tile::Cube => f.write_char('#'),
            Tile::Rounded => f.write_char('O'),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Cube,
            'O' => Self::Rounded,
            _ => panic!("unknown tile `{value}`"),
        }
    }
}

pub struct GridIterator {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl GridIterator {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for GridIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.height {
            return None;
        }
        let value = (self.x, self.y);
        if self.x + 1 < self.width {
            self.x += 1;
        } else {
            self.x = 0;
            self.y += 1;
        }

        Some(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    grid: Grid<Tile>,
}

impl Map {
    pub fn tilt(&mut self, dir: Direction) {
        // this is a naive solution where we keep iterating over the board and
        // move every rock until none can move anymore. a better solution would
        // be to iterate opposite of the move direction (e.g., if moving west,
        // iterate over columns, right-to-left) and push each rock as far as
        // they can go at one time. among other issues, this would avoid a full
        // "empty" pass of the board at the end.
        let mut running = true;
        while running {
            let mut moved = false;
            for i in 0..self.grid.len() {
                let pos = Coordinate::from_index(i, self.grid.width());
                if self.grid[pos] != Tile::Rounded {
                    continue;
                }
                moved |= self.apply_force(pos, dir);
            }
            running &= moved;
        }
    }

    pub fn spin_cycle(&mut self, mut count: usize) {
        if count == 0 {
            return;
        }

        use Direction::*;
        type Ty = Grid<Tile>;
        let dirs = [North, West, South, East];

        let mut states: HashMap<Ty, Ty> = map![];
        let mut queue = Some(vec![]);

        while count > 0 {
            count -= 1;

            // this is the meat and potatoes of the solution for high spin
            // cycles (e.g., 1 billion). we first check if this current state is
            // in the cache. if this state has a key, we can check for a "cycle"
            // or pattern of states.
            //
            // each state that is found in the cache is pushed onto the queue so
            // that we can count the length of the pattern when we come back to
            // the 0-th state. this is really wasteful on memory and can be
            // improved by splitting this up into different loops (e.g., a
            // "searching" loop and a "finish" loop after using the cycle length
            // modulus).
            //
            // for now, this solution works, but should definitely be improved.
            if let Some(grid) = states.get(&self.grid) {
                // note that this is a naive and faulty check since we don't
                // verify the full length of the cycle (e.g., this 0-th state
                // could appear several times in a single "cycle").
                if let Some(q) = queue.as_mut() {
                    if !q.is_empty() && q[0] == self.grid {
                        count %= q.len();
                        // setting the queue to None here prevents any further
                        // pattern checks. we don't need to track states after
                        // we found a cycle
                        queue = None;
                    } else {
                        q.push(self.grid.clone());
                    }
                }

                self.grid = grid.clone();
                continue;
            }

            // if the current state was not in the cache, run the actual spin
            // cycle. we can then store the result in the cache with the current
            // grid as the key.
            let key = self.grid.clone();
            dirs.iter().for_each(|&d| self.tilt(d));
            states.insert(key, self.grid.clone());
            if let Some(queue) = queue.as_mut() {
                queue.clear();
            };
        }
    }

    /// Returns the total load on the north support beam by summing the load of
    /// all [`Tile::Rounded`] tiles.
    ///
    /// The *load* is defined as the number of rows from the south edge of the
    /// platform, including the row the rock is on.
    pub fn load(&self) -> usize {
        self.grid
            .rows()
            .rev()
            .enumerate()
            .flat_map(|(i, row)| row.iter().copied().map(move |tile| (i + 1, tile)))
            .fold(0, |value, (beam, tile)| {
                value + if tile == Tile::Rounded { beam } else { 0 }
            })
    }

    /// Checks if the tile at the `pos` can be moved, and swaps it with the tile
    /// in the given `dir`.
    ///
    /// The method returns `true` if a tile was moved, and `false` otherwise.
    fn apply_force<C: Into<Coordinate>>(&mut self, pos: C, dir: Direction) -> bool {
        let c1: Coordinate = pos.into();

        if self.grid[c1] != Tile::Rounded {
            return false;
        }

        if let Some(c2) = c1.by_direction(dir) {
            if let Some(&Tile::Empty) = self.grid.get(c2) {
                self.grid.swap(c1, c2);
                return true;
            }
        }

        false
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

    static EXAMPLE_MAP: &str = "\
        O....#....\n\
        O.OO#....#\n\
        .....##...\n\
        OO.#O....O\n\
        .O.....O#.\n\
        O.#..O.#.#\n\
        ..O..#O..O\n\
        .......O..\n\
        #....###..\n\
        #OO..#....\n";

    static EXAMPLE_MAP_TILTED: &str = "\
        OOOO.#.O..\n\
        OO..#....#\n\
        OO..O##..O\n\
        O..#.OO...\n\
        ........#.\n\
        ..#....#.#\n\
        ..O..#.O.O\n\
        ..O.......\n\
        #....###..\n\
        #....#....\n";

    #[test]
    fn parse_map() {
        let m = Map::from_str(EXAMPLE_MAP).unwrap();
        assert_eq!(m.grid.width(), 10);
        assert_eq!(m.grid.height(), 10);
    }

    #[test]
    fn map_tilt() {
        let mut m = Map::from_str(EXAMPLE_MAP).unwrap();
        let expected = Map::from_str(EXAMPLE_MAP_TILTED).unwrap();

        m.tilt(Direction::North);
        assert_eq!(m.grid, expected.grid);
    }

    #[test]
    fn map_load() {
        let m = Map::from_str(EXAMPLE_MAP_TILTED).unwrap();
        assert_eq!(m.load(), 136);
    }

    #[test]
    fn map_spin_cycle() {
        let cycles = vec![
            Map::from_str(
                ".....#....\n\
                ....#...O#\n\
                ...OO##...\n\
                .OO#......\n\
                .....OOO#.\n\
                .O#...O#.#\n\
                ....O#....\n\
                ......OOOO\n\
                #...O###..\n\
                #..OO#....\n",
            )
            .unwrap(),
            Map::from_str(
                ".....#....\n\
                ....#...O#\n\
                .....##...\n\
                ..O#......\n\
                .....OOO#.\n\
                .O#...O#.#\n\
                ....O#...O\n\
                .......OOO\n\
                #..OO###..\n\
                #.OOO#...O\n",
            )
            .unwrap(),
            Map::from_str(
                ".....#....\n\
                ....#...O#\n\
                .....##...\n\
                ..O#......\n\
                .....OOO#.\n\
                .O#...O#.#\n\
                ....O#...O\n\
                .......OOO\n\
                #...O###.O\n\
                #.OOO#...O\n",
            )
            .unwrap(),
        ];

        for (i, expected) in cycles.iter().enumerate() {
            let mut m = Map::from_str(EXAMPLE_MAP).unwrap();
            m.spin_cycle(i + 1);

            assert_eq!(m, *expected, "at i={i}");
        }
    }

    #[test]
    fn map_spin_cycle_1billion() {
        let mut m = Map::from_str(EXAMPLE_MAP).unwrap();
        m.spin_cycle(1_000_000_000);

        // this load was checked with a correct submission,
        // so using this test to verify refactors
        assert_eq!(m.load(), 64);
    }
}
