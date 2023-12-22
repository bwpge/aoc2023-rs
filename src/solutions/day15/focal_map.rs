use super::{
    hash::hash,
    step::{Action, Step},
};

/// POD type to hold a [`FocalMap`] key-value pair in a bucket.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    key: String,
    value: u8,
}

impl Entry {
    fn new(k: &str, v: u8) -> Self {
        Self {
            key: k.to_owned(),
            value: v,
        }
    }
}

/// A hash map of key-value pairs for lenses.
///
/// Entries are inserted by lens label and stored in buckets determined by the
/// HASH provided by [`Hasher`].
///
/// This map can also directly execute initialization sequence steps provided by
/// [`Step`]. See the `execute` and `execute_many` methods for more information.
///
/// [`Hasher`]: super::Hasher
#[derive(Debug)]
pub struct FocalMap {
    buckets: Vec<Vec<Entry>>,
}

impl FocalMap {
    const BUCKETS: usize = 256;

    /// Creates a new [`FocalMap`].
    pub fn new() -> Self {
        Self {
            buckets: vec![vec![]; Self::BUCKETS],
        }
    }

    /// Returns `true` if the map has no entries.
    pub fn is_empty(&self) -> bool {
        self.buckets.iter().all(Vec::is_empty)
    }

    /// Inserts a value in the map, overwriting any existing value.
    pub fn insert(&mut self, k: &str, v: u8) {
        let bucket = self.bucket_mut(k);
        if let Some(idx) = bucket.iter().position(|s| s.key == k) {
            bucket[idx].value = v;
        } else {
            bucket.push(Entry::new(k, v));
        }
    }

    /// Removes a key from the map.
    ///
    /// If the key does not exist, the map is not modified.
    pub fn remove(&mut self, k: &str) {
        let bucket = self.bucket_mut(k);
        if let Some(idx) = bucket.iter().position(|s| s.key == k) {
            bucket.remove(idx);
        }
    }

    /// Executes the provided [`Step`].
    pub fn execute(&mut self, s: &Step) {
        match s.action {
            Action::Insert(v) => self.insert(&s.key, v),
            Action::Remove => self.remove(&s.key),
        }
    }

    /// Executes each [`Step`] yielded by an iterator.
    pub fn execute_many<S, It>(&mut self, it: It)
    where
        S: Into<Step>,
        It: Iterator<Item = S>,
    {
        it.for_each(|s| self.execute(&s.into()))
    }

    /// Calculates the total *focusing power* of all lenses in the map.
    ///
    /// The *focusing power* for each lens is defined as:
    ///
    /// - The 1-based index box/bucket number
    /// - The 1-based index slot number of the lens in the bucket
    /// - The focal length (entry value) of the lens
    pub fn focusing_power(&self) -> usize {
        let mut value = 0;
        for (i, bucket) in self.buckets.iter().enumerate() {
            for (j, entry) in bucket.iter().enumerate() {
                value += (i + 1) * (j + 1) * usize::from(entry.value);
            }
        }

        value
    }

    fn bucket_mut(&mut self, k: &str) -> &mut Vec<Entry> {
        &mut self.buckets[hash(k)]
    }
}

impl Default for FocalMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focal_map_insert() {
        // has the form (k, v, expected bucket, expected slot)
        let data = vec![("rn", 1, 0, 0), ("qp", 3, 1, 0), ("cm", 2, 0, 1)];
        let mut m = FocalMap::new();

        for (k, v, bucket, slot) in data {
            let entry = Entry::new(k, v);
            m.insert(k, v);
            assert_eq!(m.buckets[bucket][slot], entry, "with key={k}");
        }
    }

    #[test]
    fn focal_map_remove() {
        let mut m = FocalMap::new();

        assert!(m.buckets.iter().all(Vec::is_empty));
        m.insert("rn", 9);
        assert!(!m.buckets[0].is_empty());
        assert!(m.buckets[1..].iter().all(Vec::is_empty));
        m.remove("rn");
        assert!(m.buckets.iter().all(Vec::is_empty));
    }

    #[test]
    fn focal_map_exec_step() {
        // has the form (k, v, expected bucket, expected slot)
        let data = vec![("rn", 1, 0, 0), ("qp", 3, 1, 0), ("cm", 2, 0, 1)];
        let mut m = FocalMap::new();

        for (k, v, bucket, slot) in data {
            let entry = Entry::new(k, v);
            let step = Step::new(k, Action::Insert(v));
            m.execute(&step);
            assert_eq!(m.buckets[bucket][slot], entry, "with key={k}");
        }
    }

    #[test]
    fn focal_map_example_power() {
        let steps = vec![
            Step::new("rn", Action::Insert(1)),
            Step::new("cm", Action::Remove),
            Step::new("qp", Action::Insert(3)),
            Step::new("cm", Action::Insert(2)),
            Step::new("qp", Action::Remove),
            Step::new("pc", Action::Insert(4)),
            Step::new("ot", Action::Insert(9)),
            Step::new("ab", Action::Insert(5)),
            Step::new("pc", Action::Remove),
            Step::new("pc", Action::Insert(6)),
            Step::new("ot", Action::Insert(7)),
        ];
        let mut m = FocalMap::new();
        m.execute_many(steps.into_iter());
        assert_eq!(m.focusing_power(), 145);
    }
}
