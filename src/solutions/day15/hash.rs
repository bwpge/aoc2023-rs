/// A hash builder for the Holiday ASCII String Helper (HASH) algorithm.
#[derive(Debug, Default)]
pub struct Hasher {
    state: usize,
}

impl Hasher {
    /// Creates a new [`Hasher`] with default state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Combines the input character with the current state.
    fn combine_impl(&mut self, c: char) {
        self.state = ((self.state + (c as usize)) * 17) % 256;
    }

    /// Hashes all characters yielded by the [`Iterator`].
    ///
    /// This method modifies the internal state, and does not reset before or
    /// after hashing. To reset the internal state, use [`Hasher::finalize`].
    pub fn combine<C: Iterator<Item = char>>(&mut self, data: C) -> &mut Self {
        data.for_each(|c| self.combine_impl(c));
        self
    }

    /// Hashes all characters of the input string.
    ///
    /// For a more generic method accepting any iterator yielding `char`, see
    /// [`Hasher::combine`].
    pub fn combine_str<S: AsRef<str>>(&mut self, s: S) -> &mut Self {
        self.combine(s.as_ref().chars())
    }

    /// Returns the current state and resets it.
    pub fn finalize(&mut self) -> usize {
        let value = self.state;
        self.state = 0;
        value
    }
}

/// Shorthand to calculate the HASH of a `&str` with [`Hasher`] and return the
/// value.
///
/// A temporary [`Hasher`] is created for each invokation.
pub fn hash(s: &str) -> usize {
    Hasher::new().combine_str(s).finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hasher_combine_char() {
        let mut h = Hasher::new();
        h.combine_impl('H');

        assert_eq!(h.finalize(), 200);
    }

    #[test]
    fn hasher_combine_str() {
        let s = "HASH";
        let mut h = Hasher::new();
        h.combine(s.chars());

        assert_eq!(h.finalize(), 52);
    }

    #[test]
    fn hasher_finalize_sum() {
        let s = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let mut h = Hasher::new();

        let mut sum = 0;
        for s in s.split(',') {
            h.combine(s.chars());
            sum += h.finalize();
        }

        assert_eq!(h.state, 0);
        assert_eq!(sum, 1320);
    }
}
