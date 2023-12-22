use std::str::FromStr;

use anyhow::bail;

/// An action to be executed for an initialization sequence [`Step`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Inserts the value into the map.
    Insert(u8),
    /// Removes the value from the map.
    Remove,
}

/// An initialization sequence "step" from Appendix 1A.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    /// The lens label.
    pub key: String,
    /// The type of action to execute.
    pub action: Action,
}

impl Step {
    /// Creates a new initialization sequence [`Step`].
    pub fn new(k: &str, action: Action) -> Self {
        Self {
            key: k.to_owned(),
            action,
        }
    }
}

impl FromStr for Step {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('=') {
            let (k, v) = s.split_once('=').expect("value should have `=` delimiter");
            let action = Action::Insert(u8::from_str(v)?);
            return Ok(Self::new(k, action));
        }

        if s.ends_with('-') {
            let k = s.strip_suffix('-').expect("value should end with `-`");
            return Ok(Self::new(k, Action::Remove));
        }

        bail!("unknown initialization sequence step");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sequence() {
        let data = vec![
            ("rn=1", Step::new("rn", Action::Insert(1))),
            ("cm-", Step::new("cm", Action::Remove)),
        ];
        for (input, expected) in data {
            let s = Step::from_str(input).unwrap();
            assert_eq!(s, expected);
        }
    }
}
