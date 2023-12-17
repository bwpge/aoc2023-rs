static SPELLED_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Represents how to decode a value.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    /// Only digits (`1`, `2`, `3`, etc.)
    DigitOnly,
    /// Digits or words (`"one"`, `2`,` "three"`, etc.)
    DigitOrWord,
}

/// The decoding scan direction.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    // Scan from left-to-right.
    Forward,
    // Scan from right-to-left.
    Reverse,
}

/// Decodes the first occurence of digit matching the `mode`, in the provided
/// scan direction.
fn decode_impl(value: &str, direction: Direction, mode: Mode) -> u32 {
    let it: Box<dyn Iterator<Item = (usize, char)>> = match direction {
        Direction::Forward => Box::new(value.char_indices()),
        Direction::Reverse => Box::new(value.char_indices().rev()),
    };
    for (i, c) in it {
        if let Some(num) = c.to_digit(10) {
            return num;
        }
        if mode == Mode::DigitOrWord {
            for (num, &s) in SPELLED_DIGITS.iter().enumerate() {
                if value[i..].starts_with(s) {
                    return num as u32;
                }
            }
        }
    }

    unreachable!()
}

/// Decodes the first occurence of a digit matching the `mode`.
pub fn decode_first(value: &str, mode: Mode) -> u32 {
    decode_impl(value, Direction::Forward, mode)
}

/// Decodes the last occurence of a digit matching the `mode`.
pub fn decode_last(value: &str, mode: Mode) -> u32 {
    decode_impl(value, Direction::Reverse, mode)
}

/// A type that can have digits decoded from it.
pub trait Decode {
    /// Decodes a number from this type using the provided `mode`.
    fn decode(&self, mode: Mode) -> u32;

    /// Decodes a number from this type using [`Mode::DigitOnly`].
    fn decode_digits(&self) -> u32 {
        self.decode(Mode::DigitOnly)
    }

    /// Decodes a number from this type using [`Mode::DigitOrWord`].
    fn decode_digits_words(&self) -> u32 {
        self.decode(Mode::DigitOrWord)
    }
}

impl<'d> Decode for &'d str {
    fn decode(&self, mode: Mode) -> u32 {
        let hi = decode_first(self, mode);
        let lo = decode_last(self, mode);
        (hi * 10) + lo
    }
}

impl Decode for String {
    fn decode(&self, mode: Mode) -> u32 {
        self.as_str().decode(mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_DIGITS_ONLY: &str = "\
        1abc2\n\
        pqr3stu8vwx\n\
        a1b2c3d4e5f\n\
        treb7uchet\n";

    static EXAMPLE_DIGITS_WORDS: &str = "\
        two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen\n";

    #[test]
    fn decde_first_digit_only() {
        let values = vec![1, 3, 1, 7];

        for (input, expected) in EXAMPLE_DIGITS_ONLY.lines().zip(values) {
            let result = decode_first(input, Mode::DigitOnly);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn decde_last_digit_only() {
        let values = vec![2, 8, 5, 7];

        for (input, expected) in EXAMPLE_DIGITS_ONLY.lines().zip(values) {
            let result = decode_last(input, Mode::DigitOnly);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn decde_first_with_words() {
        let values = vec![2, 8, 1, 2, 4, 1, 7];

        for (input, expected) in EXAMPLE_DIGITS_WORDS.lines().zip(values) {
            let result = decode_first(input, Mode::DigitOrWord);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn decde_last_with_words() {
        let values = vec![9, 3, 3, 4, 2, 4, 6];

        for (input, expected) in EXAMPLE_DIGITS_WORDS.lines().zip(values) {
            let result = decode_last(input, Mode::DigitOrWord);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn trait_decode_digits_only() {
        let values = vec![12, 38, 15, 77];

        for (input, expected) in EXAMPLE_DIGITS_ONLY.lines().zip(values) {
            let result = input.decode_digits();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn trait_decode_digits_words() {
        let values = vec![29, 83, 13, 24, 42, 14, 76];

        for (input, expected) in EXAMPLE_DIGITS_WORDS.lines().zip(values) {
            let result = input.decode_digits_words();
            assert_eq!(result, expected);
        }
    }
}
