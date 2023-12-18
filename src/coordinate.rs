#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    /// Creates a new [`Coordinate`] with the provided `(x, y)`.
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Returns the coordinate directly north to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    pub fn north(&self) -> Option<Coordinate> {
        if self.y > 0 {
            return Some(Coordinate {
                x: self.x,
                y: self.y - 1,
            });
        }
        None
    }

    /// Returns the coordinate directly east to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    pub fn east(&self) -> Option<Coordinate> {
        if self.x < usize::MAX {
            return Some(Coordinate {
                x: self.x + 1,
                y: self.y,
            });
        }
        None
    }

    /// Returns the coordinate directly south to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    pub fn south(&self) -> Option<Coordinate> {
        if self.y < usize::MAX {
            return Some(Coordinate {
                x: self.x,
                y: self.y + 1,
            });
        }
        None
    }

    /// Returns the coordinate directly west to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    pub fn west(&self) -> Option<Coordinate> {
        if self.x > 0 {
            return Some(Coordinate {
                x: self.x - 1,
                y: self.y,
            });
        }
        None
    }
}

impl From<&(usize, usize)> for Coordinate {
    fn from(value: &(usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Self::from(&value)
    }
}
