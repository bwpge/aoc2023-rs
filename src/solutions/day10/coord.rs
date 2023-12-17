#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn with_coords(from: &Coord, to: &Coord) -> Option<Self> {
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
pub struct Coord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Coord {
    /// Returns the coordinate directly north to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    pub fn north(&self) -> Option<Coord> {
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
    pub fn east(&self) -> Option<Coord> {
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
    pub fn south(&self) -> Option<Coord> {
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
    pub fn west(&self) -> Option<Coord> {
        if self.x > 0 {
            return Some(Coord {
                x: self.x - 1,
                y: self.y,
            });
        }
        None
    }
}
