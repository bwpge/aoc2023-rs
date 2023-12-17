//! Solution for Advent of Code 2023, Day 10.
//!
//! # Day 10: Pipe Maze
//!
//! ## Part One
//!
//! You use the hang glider to ride the hot air from Desert Island all the way
//! up to the floating metal island. This island is surprisingly cold and there
//! definitely aren't any thermals to glide on, so you leave your hang glider
//! behind.
//!
//! You wander around for a while, but you don't find any people or animals.
//! However, you do occasionally find signposts labeled "Hot Springs" pointing
//! in a seemingly consistent direction; maybe you can find someone at the hot
//! springs and ask them where the desert-machine parts are made.
//!
//! The landscape here is alien; even the flowers and trees are made of metal.
//! As you stop to admire some metal grass, you notice something metallic scurry
//! away in your peripheral vision and jump into a big pipe! It didn't look like
//! any animal you've ever seen; if you want a better look, you'll need to get
//! ahead of it.
//!
//! Scanning the area, you discover that the entire field you're standing on is
//! densely packed with pipes; it was hard to tell at first because they're the
//! same metallic silver color as the "ground". You make a quick sketch of all
//! of the surface pipes you can see (your puzzle input).
//!
//! The pipes are arranged in a two-dimensional grid of tiles:
//!
//!   - `|` is a vertical pipe connecting north and south.
//!   - `-` is a horizontal pipe connecting east and west.
//!   - `L` is a 90-degree bend connecting north and east.
//!   - `J` is a 90-degree bend connecting north and west.
//!   - `7` is a 90-degree bend connecting south and west.
//!   - `F` is a 90-degree bend connecting south and east.
//!   - `.` is ground; there is no pipe in this tile.
//!   - `S` is the starting position of the animal; there is a pipe on this
//!     tile, but your sketch doesn't show what shape the pipe has.
//!
//! Based on the acoustics of the animal's scurrying, you're confident the pipe
//! that contains the animal is one large, continuous loop.
//!
//! For example, here is a square loop of pipe:
//!
//! ```txt
//! .....
//! .F-7.
//! .|.|.
//! .L-J.
//! .....
//! ```
//!
//! If the animal had entered this loop in the northwest corner, the sketch
//! would instead look like this:
//!
//! ```txt
//! .....
//! .S-7.
//! .|.|.
//! .L-J.
//! .....
//! ```
//!
//! In the above diagram, the S tile is still a 90-degree F bend: you can tell
//! because of how the adjacent pipes connect to it.
//!
//! Unfortunately, there are also many pipes that aren't connected to the loop!
//! This sketch shows the same loop as above:
//!
//! ```txt
//! -L|F7
//! 7S-7|
//! L|7||
//! -L-J|
//! L|-JF
//! ```
//!
//! In the above diagram, you can still figure out which pipes form the main
//! loop: they're the ones connected to S, pipes those pipes connect to, pipes
//! those pipes connect to, and so on. Every pipe in the main loop connects to
//! its two neighbors (including S, which will have exactly two pipes connecting
//! to it, and which is assumed to connect back to those two pipes).
//!
//! Here is a sketch that contains a slightly more complex main loop:
//!
//! ```txt
//! ..F7.
//! .FJ|.
//! SJ.L7
//! |F--J
//! LJ...
//! ```
//!
//! Here's the same example sketch with the extra, non-main-loop pipe tiles also
//! shown:
//!
//! ```txt
//! 7-F7-
//! .FJ|7
//! SJLL7
//! |F--J
//! LJ.LJ
//! ```
//!
//! If you want to get out ahead of the animal, you should find the tile in the
//! loop that is farthest from the starting position. Because the animal is in
//! the pipe, it doesn't make sense to measure this by direct distance. Instead,
//! you need to find the tile that would take the longest number of steps along
//! the loop to reach from the starting point - regardless of which way around
//! the loop the animal went.
//!
//! In the first example with the square loop:
//!
//! ```txt
//! .....
//! .S-7.
//! .|.|.
//! .L-J.
//! .....
//! ```
//!
//! You can count the distance each tile in the loop is from the starting point
//! like this:
//!
//! ```txt
//! .....
//! .012.
//! .1.3.
//! .234.
//! .....
//! ```
//!
//! In this example, the farthest point from the start is 4 steps away.
//!
//! Here's the more complex loop again:
//!
//! ```txt
//! ..F7.
//! .FJ|.
//! SJ.L7
//! |F--J
//! LJ...
//! ```
//!
//! Here are the distances for each tile on that loop:
//!
//! ```txt
//! ..45.
//! .236.
//! 01.78
//! 14567
//! 23...
//! ```
//!
//! Find the single giant loop starting at S. **How many steps along the loop
//! does it take to get from the starting position to the point farthest from
//! the starting position?**
//!
//! ## Part Two
//!
//! You quickly reach the farthest point of the loop, but the animal never
//! emerges. Maybe its nest is within the area enclosed by the loop?
//!
//! To determine whether it's even worth taking the time to search for such a
//! nest, you should calculate how many tiles are contained within the loop. For
//! example:
//!
//! ```txt
//! ...........
//! .S-------7.
//! .|F-----7|.
//! .||.....||.
//! .||.....||.
//! .|L-7.F-J|.
//! .|..|.|..|.
//! .L--J.L--J.
//! ...........
//! ```
//!
//! The above loop encloses merely four tiles - the two pairs of . in the
//! southwest and southeast (marked `I` below). The middle . tiles (marked `O`
//! below) are not in the loop. Here is the same loop again with those regions
//! marked:
//!
//! ```txt
//! ...........
//! .S-------7.
//! .|F-----7|.
//! .||OOOOO||.
//! .||OOOOO||.
//! .|L-7OF-J|.
//! .|II|O|II|.
//! .L--JOL--J.
//! .....O.....
//! ```
//!
//! In fact, there doesn't even need to be a full tile path to the outside for
//! tiles to count as outside the loop - squeezing between pipes is also
//! allowed! Here, `I` is still within the loop and `O` is still outside the
//! loop:
//!
//! ```txt
//! ..........
//! .S------7.
//! .|F----7|.
//! .||OOOO||.
//! .||OOOO||.
//! .|L-7F-J|.
//! .|II||II|.
//! .L--JL--J.
//! ..........
//! ```
//!
//! In both of the above examples, 4 tiles are enclosed by the loop.
//!
//! Here's a larger example:
//!
//! ```txt
//! .F----7F7F7F7F-7....
//! .|F--7||||||||FJ....
//! .||.FJ||||||||L7....
//! FJL7L7LJLJ||LJ.L-7..
//! L--J.L7...LJS7F-7L7.
//! ....F-J..F7FJ|L7L7L7
//! ....L7.F7||L7|.L7L7|
//! .....|FJLJ|FJ|F7|.LJ
//! ....FJL-7.||.||||...
//! ....L---J.LJ.LJLJ...
//! ```
//!
//! The above sketch has many random bits of ground, some of which are in the
//! loop (`I`) and some of which are outside it (`O`):
//!
//! ```txt
//! OF----7F7F7F7F-7OOOO
//! O|F--7||||||||FJOOOO
//! O||OFJ||||||||L7OOOO
//! FJL7L7LJLJ||LJIL-7OO
//! L--JOL7IIILJS7F-7L7O
//! OOOOF-JIIF7FJ|L7L7L7
//! OOOOL7IF7||L7|IL7L7|
//! OOOOO|FJLJ|FJ|F7|OLJ
//! OOOOFJL-7O||O||||OOO
//! OOOOL---JOLJOLJLJOOO
//! ```
//!
//! In this larger example, 8 tiles are enclosed by the loop.
//!
//! Any tile that isn't part of the main loop can count as being enclosed by the
//! loop. Here's another example with many bits of junk pipe lying around that
//! aren't connected to the main loop at all:
//!
//! ```txt
//! FF7FSF7F7F7F7F7F---7
//! L|LJ||||||||||||F--J
//! FL-7LJLJ||||||LJL-77
//! F--JF--7||LJLJ7F7FJ-
//! L---JF-JLJ.||-FJLJJ7
//! |F|F-JF---7F7-L7L|7|
//! |FFJF7L7F-JF7|JL---7
//! 7-L-JL7||F7|L7F-7F7|
//! L.L7LFJ|||||FJL7||LJ
//! L7JLJL-JLJLJL--JLJ.L
//! ```
//!
//! Here are just the tiles that are enclosed by the loop marked with I:
//!
//! ```txt
//! FF7FSF7F7F7F7F7F---7
//! L|LJ||||||||||||F--J
//! FL-7LJLJ||||||LJL-77
//! F--JF--7||LJLJIF7FJ-
//! L---JF-JLJIIIIFJLJJ7
//! |F|F-JF---7IIIL7L|7|
//! |FFJF7L7F-JF7IIL---7
//! 7-L-JL7||F7|L7F-7F7|
//! L.L7LFJ|||||FJL7||LJ
//! L7JLJL-JLJLJL--JLJ.L
//! ```
//!
//! In this last example, 10 tiles are enclosed by the loop.
//!
//! Figure out whether you have time to search for the nest by calculating the
//! area within the loop. **How many tiles are enclosed by the loop?**

use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::Path,
    str::FromStr,
};

use anyhow::{anyhow, bail, Result};

use crate::map;

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn with_coords(from: &Coord, to: &Coord) -> Option<Self> {
        let dx = i64::try_from(to.x).ok()? - i64::try_from(from.x).ok()?;
        let dy = i64::try_from(to.y).ok()? - i64::try_from(from.y).ok()?;

        if dy > 0 {
            return Some(Self::South);
        }
        if dy < 0 {
            return Some(Self::North);
        }
        if dx > 0 {
            return Some(Self::East);
        }
        if dx < 0 {
            return Some(Self::West);
        }

        None
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    /// Returns the coordinate directly north to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    fn north(&self) -> Option<Coord> {
        if self.y > 0 {
            return Some(Coord {
                x: self.x,
                y: self.y - 1,
            });
        }
        None
    }

    /// Returns the coordinate directly east to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    fn east(&self) -> Option<Coord> {
        if self.x < usize::MAX {
            return Some(Coord {
                x: self.x + 1,
                y: self.y,
            });
        }
        None
    }

    /// Returns the coordinate directly south to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    fn south(&self) -> Option<Coord> {
        if self.y < usize::MAX {
            return Some(Coord {
                x: self.x,
                y: self.y + 1,
            });
        }
        None
    }

    /// Returns the coordinate directly west to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    fn west(&self) -> Option<Coord> {
        if self.x > 0 {
            return Some(Coord {
                x: self.x - 1,
                y: self.y,
            });
        }
        None
    }
}

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
struct Maze {
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

    fn furthest(&self) -> u64 {
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

    fn enclosed(&self) -> u64 {
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

fn part1(maze: &Maze) {
    println!("Part 1: {}", maze.furthest());
}

fn part2(maze: &Maze) {
    println!("Part 2: {}", maze.enclosed());
}

/// Executes the solution with provided input file.
pub fn exec<P: AsRef<Path>>(path: P) -> Result<()> {
    let maze = Maze::from_str(&std::fs::read_to_string(path)?)?;

    part1(&maze);
    part2(&maze);

    Ok(())
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
