/// Represents a [cardinal direction](https://en.wikipedia.org/wiki/Cardinal_direction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Up, forward, or an azimuth of 0°
    North,
    /// Right, or an azimuth 90°
    East,
    /// Down, back, or an azimuth of 180°
    South,
    /// Left, or an azimuth of 270°
    West,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    /// Creates a new [`Coordinate`] with the provided `(x, y)`.
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Returns the equivalent one-dimensional index.
    ///
    /// This method is essentially just a wrapper for the linear equation
    /// `y = m*x + b`.
    pub fn to_index(&self, width: usize) -> usize {
        self.y * width + self.x
    }

    /// Returns a [Coordinate] based on `offset` and row `width`.
    ///
    /// # Panics
    ///
    /// This function panics if `width` is `0`.
    pub fn from_index(offset: usize, width: usize) -> Self {
        assert!(width > 0, "width must be non-zero to create xy-coordinates");
        Self::new(offset / width, offset % width)
    }

    /// Returns the coordinate directly north to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    pub fn north(&self) -> Option<Self> {
        if self.y > 0 {
            return Some(Self::new(self.x, self.y - 1));
        }
        None
    }

    /// Returns the coordinate directly east to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    pub fn east(&self) -> Option<Self> {
        if self.x < usize::MAX {
            return Some(Self::new(self.x + 1, self.y));
        }
        None
    }

    /// Returns the coordinate directly south to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    pub fn south(&self) -> Option<Self> {
        if self.y < usize::MAX {
            return Some(Self::new(self.x, self.y + 1));
        }
        None
    }

    /// Returns the coordinate directly west to this one.
    ///
    /// Returns `None` if the coordinate cannot be represented by [`usize`].
    pub fn west(&self) -> Option<Self> {
        if self.x > 0 {
            return Some(Self::new(self.x - 1, self.y));
        }
        None
    }

    /// Returns the coordinate pointed to by the given `dir`.
    ///
    /// For example, given a coordinate of `(0, 1)`, this method would return
    /// `(0, 0)` for [`Direction::North`]. Likewise, this method would return
    /// `None` for a [`Direction::West`], since `(-1, 1)` is out of bounds for
    /// a [Coordinate].
    pub fn by_direction(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::North => self.north(),
            Direction::East => self.east(),
            Direction::South => self.south(),
            Direction::West => self.west(),
        }
    }

    /// Returns the [`Direction`] pointed to by the 2D vector formed with
    /// this coordinate and the `to` position.
    ///
    /// Returns [`None`] if the coordinates are not aligned in a cardinal
    /// direction.
    pub fn direction<C: Into<Self>>(&self, to: C) -> Option<Direction> {
        let to: Coordinate = to.into();
        let dx = i64::try_from(to.x).ok()? - i64::try_from(self.x).ok()?;
        let dy = i64::try_from(to.y).ok()? - i64::try_from(self.y).ok()?;

        if dx != 0 && dy != 0 {
            return None;
        }

        if dy > 0 {
            return Some(Direction::South);
        }
        if dy < 0 {
            return Some(Direction::North);
        }
        if dx > 0 {
            return Some(Direction::East);
        }
        if dx < 0 {
            return Some(Direction::West);
        }

        None
    }
}

impl From<&Coordinate> for Coordinate {
    fn from(value: &Coordinate) -> Self {
        *value
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
