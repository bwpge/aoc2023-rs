use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use anyhow::{anyhow, bail};

use crate::map;

use super::coord::{Coord, Direction};

#[derive(Debug, Default, PartialEq, Eq)]
struct Sample {
    top_left: bool,
    top_right: bool,
    bot_left: bool,
    bot_right: bool,
}

impl Sample {
    /// Creates a new [`Sample`] with all quadrants set to `value`.
    fn new(value: bool) -> Self {
        Self {
            top_left: value,
            top_right: value,
            bot_left: value,
            bot_right: value,
        }
    }

    /// Creates a [`Sample`] from a pipe character and starting top-left fill
    /// value.
    ///
    /// The value inferred by the top-left assumes a left-to-right scan order
    /// for subsamples.
    fn with_pipe(pipe: char, top_left: bool) -> Self {
        let mut sample = Self::new(top_left);

        match pipe {
            '|' => {
                sample.top_right = !top_left;
                sample.bot_left = top_left;
                sample.bot_right = !top_left;
            }
            '-' => {
                sample.top_right = top_left;
                sample.bot_left = !top_left;
                sample.bot_right = !top_left;
            }
            'L' => {
                sample.top_right = !top_left;
                sample.bot_left = top_left;
                sample.bot_right = top_left;
            }
            'J' => {
                sample.top_right = !top_left;
                sample.bot_left = !top_left;
                sample.bot_right = !top_left;
            }
            '7' => {
                sample.top_right = top_left;
                sample.bot_left = !top_left;
                sample.bot_right = top_left;
            }
            'F' => {
                sample.top_right = top_left;
                sample.bot_left = top_left;
                sample.bot_right = !top_left;
            }
            '.' => {
                sample.top_right = top_left;
                sample.bot_left = top_left;
                sample.bot_right = top_left;
            }
            _ => panic!("unknown tile"),
        }

        sample
    }

    /// Shorthand to check if all quadrants are filled, or `true`.
    fn is_filled(&self) -> bool {
        self.top_left && self.top_right && self.bot_left && self.bot_right
    }
}

#[derive(Debug)]
pub struct Maze {
    grid: Vec<Vec<char>>,
    start: Coord,
    main_loop: HashSet<Coord>,
}

impl Maze {
    fn new(grid: Vec<Vec<char>>, start: Coord) -> Self {
        let mut maze = Self {
            grid,
            start,
            main_loop: Default::default(),
        };
        maze.trace_main_loop();
        maze.replace_start_tile();

        maze
    }

    fn at(&self, pos: &Coord) -> char {
        self.grid[pos.y][pos.x]
    }

    fn contains(&self, pos: &Coord) -> bool {
        pos.y < self.grid.len() && pos.x < self.grid[0].len()
    }

    fn adjacent(&self, pos: &Coord) -> Vec<Coord> {
        [pos.north(), pos.east(), pos.south(), pos.west()]
            .into_iter()
            .flatten()
            .filter(|c| self.contains(c) && self.is_connected(pos, c))
            .collect()
    }

    /// Checks whether or not `pos` can connect to the tile in the given
    /// direction.
    ///
    /// This method validates the boundaries of the grid and rules of pipe
    /// connections.
    fn connects_to_dir(&self, pos: &Coord, dir: Direction) -> bool {
        if !self.contains(pos) {
            return false;
        }
        let c = self.at(pos);

        if c == 'S' {
            return true;
        }

        match dir {
            Direction::North => c == '|' || c == 'L' || c == 'J',
            Direction::East => c == '-' || c == 'L' || c == 'F',
            Direction::South => c == '|' || c == '7' || c == 'F',
            Direction::West => c == '-' || c == 'J' || c == '7',
        }
    }

    /// Checks if `from` can connect to the `to` coordinate.
    fn connects(&self, from: &Coord, to: &Coord) -> bool {
        Direction::with_coords(from, to)
            .map(|d| self.connects_to_dir(from, d))
            .unwrap_or_default()
    }

    /// Checks if both `a` and `b` are connected. That is, whether or not `a`
    /// can connect to `b`, and `b` can connect to `a`.
    fn is_connected(&self, a: &Coord, b: &Coord) -> bool {
        self.connects(a, b) && self.connects(b, a)
    }

    pub fn furthest(&self) -> u64 {
        let mut distances: HashMap<Coord, u64> = map![];
        let mut nodes = VecDeque::from([(self.start.clone(), 0u64)]);

        while let Some((node, dist)) = nodes.pop_front() {
            if distances.contains_key(&node) && distances[&node] <= dist {
                continue;
            }
            distances.insert(node.clone(), dist);

            for n in self.adjacent(&node) {
                nodes.push_back((n, dist + 1));
            }
        }

        distances.values().copied().max().unwrap_or_default()
    }

    pub fn enclosed(&self) -> u64 {
        let mut total = 0;

        for (y, row) in self.grid.iter().enumerate() {
            let mut last: Option<Sample> = None;

            for (x, &c) in row.iter().enumerate() {
                let top_left = match last {
                    Some(s) => s.top_right,
                    None => false,
                };

                let next = if self.main_loop.contains(&Coord { x, y }) {
                    Sample::with_pipe(c, top_left)
                } else {
                    Sample::new(top_left)
                };

                if next.is_filled() {
                    total += 1;
                }
                last = Some(next);
            }
        }

        total
    }

    /// Traces the main loop following pipe connection rules, and stores those
    /// coordinates internally.
    fn trace_main_loop(&mut self) {
        let mut nodes = VecDeque::from([self.start.clone()]);
        let mut visited = HashSet::new();

        while let Some(node) = nodes.pop_front() {
            visited.insert(node.clone());
            self.adjacent(&node)
                .into_iter()
                .filter(|n| !visited.contains(n))
                .for_each(|n| nodes.push_back(n));
        }

        self.main_loop = visited;
    }

    fn replace_start_tile(&mut self) {
        let dirs = [
            self.start.north(),
            self.start.east(),
            self.start.south(),
            self.start.west(),
        ]
        .into_iter()
        .map(|opt| {
            opt.map(|c| self.connects(&c, &self.start))
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();

        let (n, e, s, w) = (dirs[0], dirs[1], dirs[2], dirs[3]);
        let c = match (n, e, s, w) {
            (true, false, true, false) => '|',
            (false, true, false, true) => '-',
            (true, true, false, false) => 'L',
            (true, false, false, true) => 'J',
            (false, false, true, true) => '7',
            (false, true, true, false) => 'F',
            _ => panic!("unknown starting tile from connections"),
        };
        self.grid[self.start.y][self.start.x] = c;
    }

    /// Searches the grid for an `S` tile and returns the coordinate if found.
    fn find_start_pos(grid: &[Vec<char>]) -> Option<Coord> {
        for (y, line) in grid.iter().enumerate() {
            if !line.contains(&'S') {
                continue;
            }

            for (x, &c) in line.iter().enumerate() {
                if c == 'S' {
                    return Some(Coord { x, y });
                }
            }
        }

        None
    }
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();

        // verify grid dimensions
        if grid.is_empty() || grid[0].is_empty() {
            bail!("grid must not be empty");
        }
        let width = grid[0].len();
        if !grid.iter().all(|row| row.len() == width) {
            bail!("grid must have equal width for all columns");
        }

        // find start position
        let start = Self::find_start_pos(&grid)
            .ok_or_else(|| anyhow!("grid must contain a start position"))?;

        Ok(Self::new(grid, start))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SIMPLE_PIPES: &str = "\
        .....\n\
        .S-7.\n\
        .|.|.\n\
        .L-J.\n\
        .....\n";

    static EXAMPLE_PIPES: &str = "\
        -L|F7\n\
        7S-7|\n\
        L|7||\n\
        -L-J|\n\
        L|-JF\n";

    #[test]
    fn maze_parse() {
        let maze = Maze::from_str(EXAMPLE_PIPES).unwrap();
        assert_eq!(maze.grid.len(), 5);
        assert_eq!(maze.grid[0].len(), 5);
    }

    #[test]
    fn maze_connects() {
        let maze = Maze::from_str(SIMPLE_PIPES).unwrap();
        // has the form: ((x, y), (n, e, s, w))
        let test_data = vec![
            ((0, 0), (false, false, false, false)), // .
            ((1, 1), (false, true, true, false)),   // S => deduces to F
            ((2, 1), (false, true, false, true)),   // -
            ((3, 1), (false, false, true, true)),   // 7
            ((1, 2), (true, false, true, false)),   // |
            ((3, 2), (true, false, true, false)),   // |
            ((1, 3), (true, true, false, false)),   // L
            ((2, 3), (false, true, false, true)),   // -
            ((3, 3), (true, false, false, true)),   // J
        ];

        for ((x, y), (expect_n, expect_e, expect_s, expect_w)) in test_data {
            let pos = Coord { x, y };
            let msg = format!("at pos=({}, {})", pos.x, pos.y);
            assert_eq!(
                maze.connects_to_dir(&pos, Direction::North),
                expect_n,
                "{msg}"
            );
            assert_eq!(
                maze.connects_to_dir(&pos, Direction::East),
                expect_e,
                "{msg}"
            );
            assert_eq!(
                maze.connects_to_dir(&pos, Direction::South),
                expect_s,
                "{msg}"
            );
            assert_eq!(
                maze.connects_to_dir(&pos, Direction::West),
                expect_w,
                "{msg}"
            );
        }
    }

    #[test]
    fn maze_adjacent() {
        let maze = Maze::from_str(SIMPLE_PIPES).unwrap();
        // has the form: ((x, y), [adjacent coords])
        let test_data = vec![
            ((1, 1), vec![(1, 2), (2, 1)]),
            ((2, 1), vec![(1, 1), (3, 1)]),
            ((2, 2), vec![]),
            ((3, 3), vec![(3, 2), (2, 3)]),
        ];

        for ((x, y), coords) in test_data {
            let adjacent = maze.adjacent(&Coord { x, y });
            assert_eq!(adjacent.len(), coords.len());

            for pos in coords.into_iter().map(|t| Coord { x: t.0, y: t.1 }) {
                assert!(adjacent.contains(&pos));
            }
        }
    }

    #[test]
    fn maze_furthest() {
        let data = "\
            7-F7-\n\
            .FJ|7\n\
            SJLL7\n\
            |F--J\n\
            LJ.LJ\n";
        let maze = Maze::from_str(data).unwrap();
        assert_eq!(maze.furthest(), 8);
    }

    #[test]
    fn maze_enclosed() {
        let data = "\
            FF7FSF7F7F7F7F7F---7\n\
            L|LJ||||||||||||F--J\n\
            FL-7LJLJ||||||LJL-77\n\
            F--JF--7||LJLJ7F7FJ-\n\
            L---JF-JLJ.||-FJLJJ7\n\
            |F|F-JF---7F7-L7L|7|\n\
            |FFJF7L7F-JF7|JL---7\n\
            7-L-JL7||F7|L7F-7F7|\n\
            L.L7LFJ|||||FJL7||LJ\n\
            L7JLJL-JLJLJL--JLJ.L\n";
        let maze = Maze::from_str(data).unwrap();
        assert_eq!(maze.enclosed(), 10);
    }
}
