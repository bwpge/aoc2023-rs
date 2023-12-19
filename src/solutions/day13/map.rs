use std::{
    fmt::{self, Write},
    str::FromStr,
};

use anyhow::bail;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("invalid tile character `{value}`"),
        }
    }
}

/// Represents a reflection in a [`Map`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reflection {
    Row(usize),
    Column(usize),
}

impl Reflection {
    /// Applies the summary logic specified in the problem for row and column
    /// reflections.
    ///
    /// This method is useful for iterator accumulators like `sum` or `fold`.
    pub fn summarize(self) -> usize {
        match self {
            Reflection::Row(r) => 100 * r,
            Reflection::Column(c) => c,
        }
    }
}

/// A map of rocks and ash, specialized to find reflections across rows or
/// columns.
pub struct Map {
    grid: Vec<Vec<Tile>>,
}

impl Map {
    /// Returns the number of columns in the underlying grid.
    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    /// Returns the number of rows in the underlying grid.
    pub fn height(&self) -> usize {
        self.grid.len()
    }

    /// Finds either a row or column [`Reflection`].
    pub fn find_reflection(&self) -> Reflection {
        // the problem states every map has a reflection line (either row or
        // column) -- if the row method returns None, we must be able to unwrap
        // the column value
        self.find_reflection_row().unwrap_or_else(|| {
            self.find_reflection_col()
                .expect("no row reflection found, grid must have a column reflection")
        })
    }

    /// Finds a reflection row in the map.
    ///
    /// The basic logic is to take an index into each row, then use two windows
    /// on each side of the index. We can compare slices to make sure each
    /// reflected row matches. Using iterators makes this quite simple, since we
    /// can utilize `rev` and `zip` to clamp how many rows need to match over
    /// the reflection point.
    ///
    /// The total matches we need is the minimum amount of elements contained in
    /// either of the two windows.
    fn find_reflection_row(&self) -> Option<Reflection> {
        for i in 0..self.height() {
            let top = 0..i;
            let bottom = i..self.height();

            // we can't have a reflection if either window is empty
            let required = top.len().min(bottom.len());
            if required == 0 {
                continue;
            }

            let mut matches = 0;
            for (r1, r2) in self.grid[top].iter().rev().zip(&self.grid[bottom]) {
                if r1 != r2 {
                    break;
                }
                matches += 1;
            }

            if matches == required {
                return Some(Reflection::Row(i));
            }
        }

        None
    }

    /// Finds a reflection column in the map.
    ///
    /// This method follows the same logic as the row counterpart, but is
    /// implemented slightly differently since we cannot slice columns from
    /// nested vectors. This could probably be easier with a crate like
    /// `ndarray`, but it's good practice to manually implement this logic.
    fn find_reflection_col(&self) -> Option<Reflection> {
        for i in 0..self.width() {
            let left = 0..i;
            let right = i..self.width();

            let required = left.len().min(right.len());
            if required == 0 {
                continue;
            }

            let mut matches = 0;
            for (c1, c2) in left.rev().zip(right) {
                // instead of comparing slices, we can just check if all
                // elements in each row are equal at the specified columns
                if !self.grid.iter().all(|row| row[c1] == row[c2]) {
                    break;
                }
                matches += 1;
            }

            if matches == required {
                return Some(Reflection::Column(i));
            }
        }

        None
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for c in row {
                match c {
                    Tile::Ash => f.write_char('.'),
                    Tile::Rock => f.write_char('#'),
                }?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![];
        let mut width = None;

        for line in s.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(Tile::from(c));
            }
            if let Some(w) = width {
                if w != row.len() {
                    bail!("invalid grid data");
                }
            }
            width = Some(row.len());
            grid.push(row);
        }
        if grid.is_empty() {
            bail!("invalid grid data");
        }

        Ok(Self { grid })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_MAP_V: &str = "\
        #.##..##.\n\
        ..#.##.#.\n\
        ##......#\n\
        ##......#\n\
        ..#.##.#.\n\
        ..##..##.\n\
        #.#.##.#.\n";

    static EXAMPLE_MAP_H: &str = "\
        #...##..#\n\
        #....#..#\n\
        ..##..###\n\
        #####.##.\n\
        #####.##.\n\
        ..##..###\n\
        #....#..#\n";

    #[test]
    fn parse_map() {
        for input in [EXAMPLE_MAP_H, EXAMPLE_MAP_V] {
            let m = Map::from_str(input).unwrap();
            assert_eq!(m.width(), 9);
            assert_eq!(m.height(), 7);
        }
    }

    #[test]
    fn map_find_reflection_row_col() {
        // has the form (input, expected row reflect, expected col reflect)
        let data = vec![
            (EXAMPLE_MAP_H, Some(Reflection::Row(4)), None),
            (EXAMPLE_MAP_V, None, Some(Reflection::Column(5))),
        ];

        for (input, expected_r, expected_c) in data {
            let m = Map::from_str(input).unwrap();
            assert_eq!(m.find_reflection_row(), expected_r);
            assert_eq!(m.find_reflection_col(), expected_c);
        }
    }

    #[test]
    fn map_find_reflection_summarize() {
        let data = vec![(EXAMPLE_MAP_H, 400), (EXAMPLE_MAP_V, 5)];

        for (input, expected) in data {
            let m = Map::from_str(input).unwrap();
            assert_eq!(m.find_reflection().summarize(), expected);
        }
    }
}
