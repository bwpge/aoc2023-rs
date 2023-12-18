use std::{collections::HashSet, str::FromStr};

use anyhow::bail;

use crate::{set, Coordinate};

pub struct GalaxyMap {
    galaxies: Vec<Coordinate>,
    expansion_factor: usize,
    expanded_rows: Vec<usize>,
    expanded_cols: Vec<usize>,
}

impl GalaxyMap {
    pub fn set_expansion(&mut self, factor: usize) {
        self.expansion_factor = factor;
    }

    pub fn galaxy_coords(&self) -> &[Coordinate] {
        &self.galaxies
    }

    pub fn galaxy_pairs(&self) -> HashSet<(Coordinate, Coordinate)> {
        let mut values = set![];
        for (i, c1) in self.galaxies.iter().enumerate() {
            for c2 in &self.galaxies[(i + 1)..] {
                values.insert((c1.clone(), c2.clone()));
            }
        }

        values
    }

    pub fn distance(&self, from: &Coordinate, to: &Coordinate) -> usize {
        let lo_x = from.x.min(to.x);
        let hi_x = from.x.max(to.x);
        let lo_y = from.y.min(to.y);
        let hi_y = from.y.max(to.y);
        let distance = (hi_y - lo_y) + (hi_x - lo_x);

        if self.expansion_factor == 0 {
            return distance;
        }

        let count = self.expanded_crossings(from, to);
        let factor = if self.expansion_factor == 1 {
            1
        } else {
            self.expansion_factor - 1
        };

        distance + (count * factor)
    }

    fn expanded_crossings(&self, from: &Coordinate, to: &Coordinate) -> usize {
        let mut count = 0;

        count += self
            .expanded_rows
            .iter()
            .filter(|&&row| Self::crosses_row(row, from, to))
            .count();
        count += self
            .expanded_cols
            .iter()
            .filter(|&&col| Self::crosses_col(col, from, to))
            .count();

        count
    }

    fn crosses_row(row: usize, from: &Coordinate, to: &Coordinate) -> bool {
        let lo = from.y.min(to.y);
        let hi = from.y.max(to.y);
        lo < row && row < hi
    }

    fn crosses_col(col: usize, from: &Coordinate, to: &Coordinate) -> bool {
        let lo = from.x.min(to.x);
        let hi = from.x.max(to.x);
        lo < col && col < hi
    }
}

impl FromStr for GalaxyMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            bail!("galaxy data must not be empty")
        }

        let mut grid = vec![];
        let mut galaxies = vec![];
        let mut expanded_rows = vec![];
        let mut expanded_cols = vec![];

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::with_capacity(line.len());
            if line.chars().all(|c| c == '.') {
                expanded_rows.push(y);
            }

            for (x, c) in line.char_indices() {
                row.push(c);
                if c == '#' {
                    galaxies.push(Coordinate::new(x, y));
                }
            }
            grid.push(row);
        }

        if grid.is_empty() || grid.iter().all(|row| row.len() != grid[0].len()) {
            bail!("invalid galaxy map");
        }

        for i in 0..grid[0].len() {
            if grid.iter().all(|row| row[i] == '.') {
                expanded_cols.push(i);
            }
        }

        Ok(Self {
            galaxies,
            expansion_factor: 1,
            expanded_rows,
            expanded_cols,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_MAP: &str = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....\n";

    macro_rules! setup {
        () => {
            GalaxyMap::from_str(EXAMPLE_MAP).unwrap()
        };
    }

    #[test]
    fn galaxy_map_parse() {
        let g = setup!();
        let expanded_rows = vec![3, 7];
        let expanded_cols = vec![2, 5, 8];

        assert_eq!(g.galaxies.len(), 9);
        assert_eq!(g.expanded_rows.len(), expanded_rows.len());
        for row in expanded_rows {
            assert!(g.expanded_rows.contains(&row));
        }
        assert_eq!(g.expanded_cols.len(), expanded_cols.len());
        for col in expanded_cols {
            assert!(g.expanded_cols.contains(&col))
        }
    }

    #[test]
    fn galaxy_map_coords() {
        let g = setup!();
        let coords = vec![
            Coordinate::new(0, 2),
            Coordinate::new(0, 9),
            Coordinate::new(3, 0),
            Coordinate::new(1, 5),
            Coordinate::new(6, 4),
            Coordinate::new(4, 9),
            Coordinate::new(9, 6),
            Coordinate::new(7, 1),
            Coordinate::new(7, 8),
        ];

        assert_eq!(g.galaxy_coords().len(), coords.len());
        for expected in coords {
            assert!(g.galaxy_coords().contains(&expected));
        }
    }

    #[test]
    fn galaxy_map_pairs() {
        let g = setup!();
        let pairs = g.galaxy_pairs();

        assert_eq!(pairs.len(), 36);
        for (c1, c2) in &pairs {
            // ensure no pairs exist with swapped coords
            assert!(!pairs.contains(&(c2.clone(), c1.clone())));
        }
    }

    #[test]
    fn galaxy_map_distance() {
        let mut g = setup!();
        let pairs = vec![(1, 374), (10, 1030), (100, 8410)];

        for (factor, expected) in pairs {
            g.set_expansion(factor);
            let sum = g
                .galaxy_pairs()
                .iter()
                .fold(0, |value, (from, to)| value + g.distance(from, to));
            assert_eq!(sum, expected);
        }
    }
}
