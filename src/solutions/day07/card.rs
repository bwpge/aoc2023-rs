use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleSet {
    faces: Vec<char>,
    wildcards: Vec<char>,
}

impl RuleSet {
    pub fn standard() -> Self {
        Self {
            faces: vec![
                '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
            ],
            wildcards: Vec::new(),
        }
    }

    pub fn jokers_wild() -> Self {
        Self {
            faces: vec![
                'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
            ],
            wildcards: vec!['J'],
        }
    }

    pub fn is_wild(&self, card: &Card) -> bool {
        if self.wildcards.is_empty() || card.value > self.faces.len() {
            return false;
        }
        self.is_wild_face(self.faces[card.value])
    }

    fn is_wild_face(&self, value: char) -> bool {
        self.wildcards.contains(&value)
    }
}

impl Default for RuleSet {
    fn default() -> Self {
        Self::standard()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card {
    pub(crate) value: usize,
}

impl Card {
    pub fn with_rules(value: char, rules: &RuleSet) -> Result<Self> {
        Ok(Self {
            value: rules
                .faces
                .iter()
                .position(|c| *c == value)
                .ok_or_else(|| anyhow!("invalid card face `{value}`"))?,
        })
    }
}

impl From<usize> for Card {
    fn from(value: usize) -> Self {
        Self { value }
    }
}
