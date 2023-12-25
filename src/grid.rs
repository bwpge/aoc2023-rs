use std::{
    fmt::{self, Write},
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::{bail, Result};
use num::range_step;

use crate::Coordinate;

/// An iterator that yields each column of the underlying [Grid].
pub struct ColumnIter<'a, T> {
    grid: &'a Grid<T>,
    idx: usize,
    end: usize,
}

impl<'a, T> ColumnIter<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            idx: 0,
            end: grid.width(),
        }
    }
}

impl<'a, T> Iterator for ColumnIter<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.end {
            return None;
        }
        let val = self.grid.column(self.idx);
        self.idx += 1;

        Some(val)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let value = if self.end < self.idx {
            0
        } else {
            self.end - self.idx
        };

        (value, Some(value))
    }
}

impl<'a, T> DoubleEndedIterator for ColumnIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.end -= 1;
        if self.idx >= self.end {
            return None;
        }

        Some(self.grid.column(self.idx))
    }
}

impl<'a, T> ExactSizeIterator for ColumnIter<'a, T> {}

/// A two-dimensional data structure used to represent maps, game boards, or any
/// other aligned cells of any data.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    inner: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    /// Creates a new [Grid] from an iterator and a `width`.
    ///
    /// # Panics
    ///
    /// Panics if `width` is `0`, or if the number of elements is not divisible
    /// by `width`.
    pub fn new<It>(it: It, width: usize) -> Self
    where
        It: Iterator<Item = T>,
    {
        debug_assert!(width > 0);
        let inner = it.collect::<Vec<_>>();
        debug_assert!(inner.len() % width == 0);

        Self { inner, width }
    }

    /// Returns the number of columns in the grid.
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the number of rows in the grid.
    #[inline]
    pub fn height(&self) -> usize {
        self.inner.len() / self.width
    }

    /// Returns the total number of elements in the grid.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the grid contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the element at the given coordinate, if it exists.
    ///
    /// To avoid bounds checking, index directly with the `[]` operator
    /// ([`Index`] or [`IndexMut`]). For the index operators, these bounds are
    /// only checked with [`debug_assert`].
    pub fn get<C: Into<Coordinate>>(&self, pos: C) -> Option<&T> {
        let c: Coordinate = pos.into();
        if c.x >= self.width() || c.y >= self.height() {
            return None;
        }

        self.inner.get(c.to_index(self.width))
    }

    /// Checks if the provided coordinate is within the grid bounds.
    ///
    /// This method is equivalent to `grid.get(pos).is_some()`. If you are
    /// checking coordinate boundaries and then immediately indexing (e.g., `if
    /// grid.contains(c) && grid[c] == foo`), it may be more prudent to use
    /// [`Grid::get`] and use the [`Option`] directly.
    #[inline]
    pub fn contains<C: Into<Coordinate>>(&self, pos: C) -> bool {
        self.get(pos).is_some()
    }

    //
    pub fn swap<A: Into<Coordinate>, B: Into<Coordinate>>(&mut self, a: A, b: B) {
        let c1: Coordinate = a.into();
        let c2: Coordinate = b.into();

        self.inner
            .swap(c1.to_index(self.width), c2.to_index(self.width));
    }

    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.inner.iter()
    }

    /// Creates an iterator over all elements in the grid in one dimension.
    ///
    /// Elements are yielded from left-to-right, top-to-bottom.
    pub fn enumerate(&self) -> impl DoubleEndedIterator + ExactSizeIterator + '_ {
        self.inner.iter().enumerate()
    }

    /// Creates an iterator over all grid elements which yields the current
    /// iteration [`Coordinate`] as well as the next value.
    pub fn enumerate_coords(
        &self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator + Iterator<Item = (Coordinate, &'_ T)> {
        self.inner
            .iter()
            .enumerate()
            .map(|(idx, el)| (Coordinate::from_index(idx, self.width), el))
    }

    /// Returns the row at the given `index`.
    ///
    /// # Panics
    ///
    /// Panics if the row `index` is out of bounds.
    pub fn row(&self, index: usize) -> &[T] {
        assert!(index < self.height());
        let start = index * self.width;
        let end = start + self.width;

        &self.inner[start..end]
    }

    /// Returns an iterator over the rows of the grid.
    pub fn rows(&self) -> impl DoubleEndedIterator + ExactSizeIterator<Item = &'_ [T]> + '_ {
        self.inner.chunks(self.width)
    }

    /// Returns the column at the given `index`.
    ///
    /// If the cell type `T` implements [`Copy`], you may want to use
    /// [`Grid::column_copied`] instead.
    ///
    /// # Panics
    ///
    /// Panics if the column `index` is out of bounds.
    pub fn column(&self, index: usize) -> Vec<&T> {
        assert!(index < self.width());

        let mut vals = Vec::with_capacity(self.height());
        for i in range_step(index, self.len(), self.width()) {
            vals.push(&self.inner[i]);
        }

        vals
    }

    /// Returns an iterator over the columns of the grid.
    ///
    /// Note that this method does require allocating a [`Vec<&T>`] with `N`
    /// elements (where `N` is the column size). There is no way to provide
    /// slices over non-contiguous memory. However, these are lazily allocated
    /// so only one column of references is created at a time.
    pub fn columns(&self) -> ColumnIter<'_, T> {
        ColumnIter::new(self)
    }
}

impl<T: Default> Grid<T> {
    /// Creates a new [`Grid`] with the specified dimensions, using the
    /// `T::default()` for cell values.
    pub fn new_default(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut inner = Vec::with_capacity(size);
        (0..size).for_each(|_| inner.push(T::default()));

        Self { inner, width }
    }
}

impl<T: Copy> Grid<T> {
    /// Returns a copy of the column at the given `index`.
    pub fn column_copied(&self, index: usize) -> Vec<T> {
        assert!(index < self.height());

        let mut vals = Vec::with_capacity(self.height());
        for i in range_step(index, self.len(), self.width()) {
            vals.push(self.inner[i]);
        }

        vals
    }
}

impl<T> Index<Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        debug_assert!(index.x < self.width() && index.y < self.height());
        &self.inner[index.to_index(self.width)]
    }
}

impl<T> IndexMut<Coordinate> for Grid<T> {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        debug_assert!(index.x < self.width() && index.y < self.height());
        &mut self.inner[index.to_index(self.width)]
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        debug_assert!(index.0 < self.width() && index.1 < self.height());
        &self[Coordinate::from(index)]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        debug_assert!(index.0 < self.width() && index.1 < self.height());
        &mut self[Coordinate::from(index)]
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, value) in self.inner.iter().enumerate() {
            if i > 0 && i % self.width() == 0 {
                f.write_char('\n')?;
            }
            value.fmt(f)?;
        }

        Ok(())
    }
}

impl<T: fmt::Display> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Grid {")?;
        for (i, value) in self.inner.iter().enumerate() {
            if i % self.width() == 0 {
                f.write_str("\n    ")?;
            }
            write!(f, "{value}")?;
        }
        f.write_str("\n}")
    }
}

impl<T: From<char>> FromStr for Grid<T> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = vec![];
        let mut width = None;

        for line in s.lines() {
            let mut w = 0;
            for c in line.chars() {
                inner.push(T::from(c));
                w += 1;
            }

            if let Some(width) = width {
                if width != w {
                    bail!(
                        "grid data contains inconsistent row \
                        sizes (expected {width}, got {w})"
                    );
                }
            }
            width = Some(w);
        }

        if inner.is_empty() {
            bail!("grid must not be empty");
        }

        Ok(Self {
            inner,
            width: width.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_3X2: &str = "012\n345\n";

    static ALPHABET_4X6: [[char; 4]; 6] = [
        ['a', 'b', 'c', 'd'],
        ['e', 'f', 'g', 'h'],
        ['i', 'j', 'k', 'l'],
        ['m', 'n', 'o', 'p'],
        ['q', 'r', 's', 't'],
        ['u', 'v', 'w', 'x'],
    ];

    #[test]
    fn grid_new() {
        let g = Grid::new(ALPHABET_4X6.iter().flatten(), 4);
        assert_eq!(g.width(), 4);
        assert_eq!(g.height(), 6);
        assert_eq!(g.len(), 24);
    }

    #[test]
    fn parse_grid() {
        let g: Grid<char> = INPUT_3X2.parse().unwrap();
        assert_eq!(g.width(), 3);
        assert_eq!(g.height(), 2);
        assert_eq!(g.len(), 6);

        for y in 0..g.height() {
            for x in 0..g.width() {
                let coord = Coordinate::new(x, y);
                let c = char::from_digit(coord.to_index(g.width()) as u32, 10).unwrap();
                assert_eq!(g[coord], c, "at ({x}, {y})");
            }
        }
    }

    #[test]
    fn grid_index() {
        let mut g = Grid::from_str(INPUT_3X2).unwrap();
        let c = Coordinate::new(1, 1);

        assert_ne!(g[c], 'X');
        g[c] = 'X';
        assert_eq!(*g.get(c).unwrap(), 'X');
    }

    #[test]
    fn grid_row_col() {
        let cols = vec![
            vec![&'a', &'e', &'i', &'m', &'q', &'u'],
            vec![&'b', &'f', &'j', &'n', &'r', &'v'],
            vec![&'c', &'g', &'k', &'o', &'s', &'w'],
            vec![&'d', &'h', &'l', &'p', &'t', &'x'],
        ];
        let g = Grid::new(ALPHABET_4X6.into_iter().flatten(), 4);

        for i in 0..ALPHABET_4X6.len() {
            assert_eq!(g.row(i), ALPHABET_4X6[i]);
        }
        for i in 0..4 {
            assert_eq!(g.column(i), cols[i]);
        }
    }

    #[test]
    fn grid_rows() {
        let g = Grid::new(ALPHABET_4X6.into_iter().flatten(), 4);
        let it = g.rows();
        assert_eq!(it.len(), ALPHABET_4X6.len());

        for (row, expected) in g.rows().zip(ALPHABET_4X6) {
            assert_eq!(row, expected);
        }
    }

    #[test]
    fn grid_columns() {
        let cols = vec![
            vec![&'a', &'e', &'i', &'m', &'q', &'u'],
            vec![&'b', &'f', &'j', &'n', &'r', &'v'],
            vec![&'c', &'g', &'k', &'o', &'s', &'w'],
            vec![&'d', &'h', &'l', &'p', &'t', &'x'],
        ];
        let g = Grid::new(ALPHABET_4X6.into_iter().flatten(), 4);
        let it = g.columns();
        assert_eq!(it.len(), cols.len());

        for (col, expected) in it.zip(cols) {
            assert_eq!(col, expected);
        }
    }
}
