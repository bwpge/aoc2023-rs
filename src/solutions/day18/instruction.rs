use std::str::FromStr;

use anyhow::{bail, Result};

use crate::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub dir: Direction,
    pub count: i64,
}

impl Instruction {
    /// Decodes a series of instructions with the [`Decoder`] type.
    pub fn decode_many<'d, D, It>(input: It, _d: D) -> Result<Vec<Self>>
    where
        It: Iterator<Item = &'d str>,
        D: Decoder + 'd,
    {
        D::decode_many(input)
    }

    /// Simulates this [`Instruction`] from the given coordinates and returns
    /// the final position as `(x, y)`.
    pub fn simulate(&self, mut x: i64, mut y: i64) -> (i64, i64) {
        use Direction::*;
        match self.dir {
            North => y -= self.count,
            East => x += self.count,
            South => y += self.count,
            West => x -= self.count,
        }

        (x, y)
    }
}

pub trait Decoder {
    fn decode(s: &str) -> Result<Instruction>;

    fn decode_many<'d, It>(it: It) -> Result<Vec<Instruction>>
    where
        It: Iterator<Item = &'d str>,
    {
        it.map(|s| Self::decode(s)).collect()
    }
}

/// Decodes instructions with standard form.
///
/// Standard form is `DIRECTION COUNT COLOR=(#RRGGBB)`
pub struct Standard;

impl Decoder for Standard {
    fn decode(s: &str) -> Result<Instruction> {
        let splits = s.trim().split(' ').collect::<Vec<_>>();
        if splits.len() != 3 {
            bail!("invalid instruction format");
        }

        let dir = match splits[0] {
            "U" => Direction::North,
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            _ => bail!("unknown direction code"),
        };
        let count = i64::from_str(splits[1])?;

        Ok(Instruction { dir, count })
    }
}

/// Decodes instructions with buggy hex form.
///
/// Buggy hex form uses only the hexadecimal value:
///
/// - The first five hexadecimal digits encode the distance in meters as a
///   five-digit hexadecimal number
/// - The last hexadecimal digit encodes the direction to dig: `0` means `R`,
///   `1` means `D`, `2` means `L`, and 3 means `U`
pub struct Hex;

impl Decoder for Hex {
    fn decode(s: &str) -> Result<Instruction> {
        let splits = s.trim().split(' ').collect::<Vec<_>>();
        if splits.len() != 3 || splits[2].len() != 9 {
            bail!("invalid instruction format");
        }
        let mut s = splits[2][2..splits[2].len() - 1].to_string();
        if s.len() != 6 {
            bail!("invalid hex format");
        }
        let dir = match s.pop().expect("string should not be empty") {
            '0' => Direction::East,
            '1' => Direction::South,
            '2' => Direction::West,
            '3' => Direction::North,
            _ => bail!("unknown direction code"),
        };
        let count = i64::from_str_radix(&s, 16)?;

        Ok(Instruction { dir, count })
    }
}

/// Calculates the lava capacity for a set of instructions.
///
/// This function uses a combination of the [Shoelace formula] and [Pick's
/// theorem] to determine the area enclosed by the input instructions. The
/// complexity comes from trying reason about area vs. border in the shape to
/// get a proper capacity.
///
/// # Inner area
///
/// Assuming the trench starts digging at `(0, 0)`, this allows us to simplify
/// the implementation by skipping the first and last terms (`x0*y1 - x1*y0` and
/// `xn*y0 - x0*yn`). Without having to consider wrapping around, this becomes a
/// very simple `fold` over all the vertices.
///
/// Since we are executing instructions in serial, we also know the coordinates
/// are oriented in either clockwise or counter-clockwise order. Summing each
/// term may yield a positive or negative value, and we can simply take the
/// absolute value to get the proper area.
///
/// # Perimeter area
///
/// An additional wrinkle is the perimeter, which must be counted as part of the
/// total capacity. We can keep track of the perimeter in the same fold
/// operation by simply subtracting the new from old coordinates and taking the
/// absolute value. With Pick's theorem, `A = I + B/2 - 1`:
///
/// - `I` is the shoelace value, or `fold(area) / 2`
/// - `B` is the boundary, or `fold(perim) + ??` (see next section)
/// - `A = (fold(area) + fold(perim)) / 2 - 1`
///
/// # Corners
///
/// The final issue to deal with is the corners on boundaries, which break from
/// the center of the cell. If we go back to `B` above, it's the number of
/// points on the boundary. However, we don't have a neat way to account for the
/// boundary points. What we are actually counting in the fold operation is the
/// perimeter, which is not accounting for the extra length of corners. Getting
/// a perimeter from coordinates is assuming the corners break on exactly the
/// same "edge" of a tile.
///
/// ```txt
/// ┌────┬────┐
/// │    │ XX │ <-- perimeter edge contributes extra length
/// │    └────┤
/// │         │
/// └─────────┘
/// ```
///
/// By simply running some unit tests, it's clear the above `A` is always off by
/// 2. We know the shape is going to be a loop (otherwise we wouldn't have an
/// inner area to work with). Moreover, we are always going to have a polygon
/// with right-angle corners. Consider some arbitrary shapes we can make:
///
/// ```txt
///  ┌───┐       ┌────┐   ┌────┐
///  │   │       └┐   │   │    │
/// ┌┘   └────┐  ┌┘  ┌┘   │    │
/// │  ┌──────┘  │   └┐   │    │
/// └──┘         └────┘   └────┘
/// ```
///
/// For all of these shapes, there are always 4 more outer corners than inner
/// corners. This is obviously not a mathematical proof, but is good enough to
/// reason about what is happening. If we use change Pick's theorem to look like
/// `A = I + (B - 2)/2`, then we can substitue our perimeter length `L + 4`
/// (accounting for 4 outer corners) for `B`.
///
/// This gives us `A = I + (L + 4 - 2)/2 = I + L/2 + 1`. This makes our fold
/// implementation quite simple:
///
/// ```txt
/// fold(instructions): initial=(area=0, perim=0, pos=(0,0))
///   - next_pos <- simulate instruction(last_pos)
///   - next_area <- last_area + shoelace(last_pos, next_pos)
///   - next perim <- last_perim + abs(next_pos - last_pos)
///
/// --> returns (area, perim, pos)
/// ```
///
/// Thus the final value is `(abs(area) + pos) / 2 + 1`.
///
/// ### Sources
///
/// - <https://dev.to/nickymeuleman/advent-of-code-2023-day-18-2041>
/// - <https://www.reddit.com/r/adventofcode/comments/18lj7wx/comment/kdz5a7v/>
///
/// [Shoelace formula]: <https://en.wikipedia.org/wiki/Shoelace_formula>
/// [Pick's theorem]: <https://en.wikipedia.org/wiki/Pick%27s_theorem>
pub fn capacity<'a, It>(it: It) -> usize
where
    It: Iterator<Item = &'a Instruction>,
{
    // fold value has the form (area, perim, x, y)
    let (a, p, _, _) = it.fold((0, 0, 0, 0), |(a, p, x, y), inst| {
        let (x1, y1) = inst.simulate(x, y);
        let a_next = a + ((x * y1) - (y * x1));
        let p_next = p + (x1 - x).abs() + (y1 - y).abs();
        (a_next, p_next, x1, y1)
    });

    let value = (a.abs() + p) / 2 + 1;
    assert!(value >= 0);

    value as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_DATA: &str = "\
        R 6 (#70c710)\n\
        D 5 (#0dc571)\n\
        L 2 (#5713f0)\n\
        D 2 (#d2c081)\n\
        R 2 (#59c680)\n\
        D 2 (#411b91)\n\
        L 5 (#8ceee2)\n\
        U 2 (#caa173)\n\
        L 1 (#1b58a2)\n\
        U 2 (#caa171)\n\
        R 2 (#7807d2)\n\
        U 3 (#a77fa3)\n\
        L 2 (#015232)\n\
        U 2 (#7a21e3)\n";

    macro_rules! dir {
        (U) => {
            Direction::North
        };
        (R) => {
            Direction::East
        };
        (D) => {
            Direction::South
        };
        (L) => {
            Direction::West
        };
    }

    macro_rules! inst {
        ($d:tt $num:literal) => {
            Instruction {
                dir: dir!($d),
                count: $num,
            }
        };
        ($($d:tt $num:literal),+ $(,)?) => {
            vec![$(Instruction {
                dir: dir!($d),
                count: $num,
            }),+]
        };
    }

    #[test]
    fn decode_standard() {
        let s = "D 2 (#411b91)";
        let inst = Standard::decode(s).unwrap();

        assert_eq!(inst.dir, Direction::South);
        assert_eq!(inst.count, 2);
    }

    #[test]
    fn decode_hex() {
        let s = "D 2 (#411b91)";
        let inst = Hex::decode(s).unwrap();

        assert_eq!(inst.dir, Direction::South);
        assert_eq!(inst.count, 266681);
    }

    #[test]
    fn decode_many_standard() {
        let expected = inst![
            R 6,
            D 5,
            L 2,
            D 2,
            R 2,
            D 2,
            L 5,
            U 2,
            L 1,
            U 2,
            R 2,
            U 3,
            L 2,
            U 2,
        ];
        let decoded = Standard::decode_many(EXAMPLE_DATA.lines()).unwrap();
        assert_eq!(decoded.len(), expected.len());

        for (decoded, expected) in decoded.into_iter().zip(expected) {
            assert_eq!(decoded, expected);
        }
    }

    #[test]
    fn decode_many_hex() {
        let expected = inst![
            R 461937,
            D 56407,
            R 356671,
            D 863240,
            R 367720,
            D 266681,
            L 577262,
            U 829975,
            L 112010,
            D 829975,
            L 491645,
            U 686074,
            L 5411,
            U 500254,
        ];

        let decoded = Hex::decode_many(EXAMPLE_DATA.lines()).unwrap();
        assert_eq!(decoded.len(), expected.len());

        for (decoded, expected) in decoded.into_iter().zip(expected) {
            assert_eq!(decoded, expected);
        }
    }

    #[test]
    fn lava_capacity_standard() {
        let i = Instruction::decode_many(EXAMPLE_DATA.lines(), Standard).unwrap();
        assert_eq!(capacity(i.iter()), 62);
    }

    #[test]
    fn lava_capacity_hex() {
        let i = Instruction::decode_many(EXAMPLE_DATA.lines(), Hex).unwrap();
        assert_eq!(capacity(i.iter()), 952408144115);
    }
}
