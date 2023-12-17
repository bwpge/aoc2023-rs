use std::str::FromStr;

use anyhow::bail;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Span {
    line: usize,
    start: usize,
    end: usize,
}

impl Span {
    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        let lo_x = if self.start == 0 { 0 } else { self.start - 1 };
        let hi_x = self.end;
        let lo_y = if self.line == 0 { 0 } else { self.line - 1 };
        let hi_y = self.line + 1;

        lo_x <= x && x <= hi_x && lo_y <= y && y <= hi_y
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Gear {
    parts: [u64; 2],
}

impl Gear {
    pub fn ratio(&self) -> u64 {
        self.parts[0] * self.parts[1]
    }
}

#[derive(Debug)]
pub struct Schematic {
    grid: Vec<Vec<char>>,
    numbers: Vec<Span>,
    symbols: Vec<(usize, usize)>,
}

impl Schematic {
    fn at(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn span_value(&self, part: &Span) -> Option<u64> {
        let s: String = self.grid[part.line][part.start..part.end].iter().collect();
        u64::from_str(&s).ok()
    }

    fn is_part(&self, span: &Span) -> bool {
        self.symbols.iter().any(|&(x, y)| span.is_adjacent(x, y))
    }

    fn parts(&self) -> Vec<&Span> {
        self.numbers.iter().filter(|&s| self.is_part(s)).collect()
    }

    pub fn part_numbers(&self) -> Vec<u64> {
        self.parts()
            .into_iter()
            .filter_map(|s| self.span_value(s))
            .collect()
    }

    pub fn gears(&self) -> Vec<Gear> {
        let mut list = vec![];
        for &(x, y) in &self.symbols {
            if self.at(x, y) != '*' {
                continue;
            }
            let adjacent = self
                .numbers
                .iter()
                .filter(|s| s.is_adjacent(x, y))
                .collect::<Vec<_>>();
            if adjacent.len() == 2 {
                let p1 = self
                    .span_value(adjacent[0])
                    .expect("span should be a part number");
                let p2 = self
                    .span_value(adjacent[1])
                    .expect("span should be a part number");
                list.push(Gear { parts: [p1, p2] });
            }
        }

        list
    }
}

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.is_empty() {
            bail!("grid data must have at least one line")
        }

        let mut grid = vec![];
        let mut parts = vec![];
        let mut symbols = vec![];

        let mut span: Option<Span> = None;
        for (j, line) in s.lines().enumerate() {
            if line.is_empty() {
                bail!("grid line must not be empty");
            }
            grid.push(line.chars().collect());
            let width = line.len();

            for (i, c) in line.char_indices() {
                // track the first digit of the number as we iterate
                if c.is_ascii_digit() {
                    if span.is_none() {
                        span = Some(Span {
                            line: j,
                            start: i,
                            end: i,
                        });
                    }
                    continue;
                }
                // complete the span if there is one tracked
                if let Some(mut span) = span.take() {
                    span.end = i;
                    parts.push(span);
                }
                // track symbol location regardless of current span
                if c != '.' {
                    symbols.push((i, j));
                }
            }
            // a span can't cross lines, so complete the existing one
            if let Some(mut span) = span.take() {
                span.end = width;
                parts.push(span);
            }
        }

        Ok(Self {
            grid,
            numbers: parts,
            symbols,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SCHEMATIC_DATA: &str = "\
        467..114.1\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..\n";

    #[test]
    fn parse_schematic() {
        let schematic = Schematic::from_str(SCHEMATIC_DATA).unwrap();
        assert_eq!(schematic.numbers.len(), 11);
        assert_eq!(schematic.symbols.len(), 6);
    }

    #[test]
    fn parse_spans() {
        let nums = vec![467, 114, 1, 35, 633, 617, 58, 592, 755, 664, 598];
        let schematic = Schematic::from_str(SCHEMATIC_DATA).unwrap();

        assert_eq!(schematic.numbers.len(), nums.len());
        for (s, expected) in schematic.numbers.iter().zip(nums) {
            let value = schematic.span_value(s).unwrap();
            assert_eq!(value, expected);
        }
    }

    #[test]
    fn schematic_part_numbers() {
        let nums = vec![467, 35, 633, 617, 592, 755, 664, 598];
        let schematic = Schematic::from_str(SCHEMATIC_DATA).unwrap();
        let part_numbers = schematic.part_numbers();

        assert_eq!(part_numbers.len(), nums.len());
        for (part, expected) in part_numbers.into_iter().zip(nums) {
            assert_eq!(part, expected);
        }
    }

    #[test]
    fn schematic_gears() {
        let nums = vec![Gear { parts: [467, 35] }, Gear { parts: [755, 598] }];
        let schematic = Schematic::from_str(SCHEMATIC_DATA).unwrap();
        let gears = schematic.gears();

        assert_eq!(gears.len(), nums.len());
        for (gear, expected) in gears.into_iter().zip(nums) {
            assert_eq!(gear, expected);
        }
    }
}
