//! Additional utilities for files, I/O, etc.

use std::{fs::File, io::BufRead, io::BufReader, path::Path, str::FromStr};

use anyhow::Result;

/// Short-hand to open a file in read-only mode and wrap it in a [`BufReader`].
pub fn buf_reader<P: AsRef<Path>>(path: P) -> Result<BufReader<File>> {
    let file = File::open(path)?;

    Ok(BufReader::new(file))
}

/// Applies the provided function to each line of a file and collects the
/// results in a [`Vec`].
pub fn map_file_lines<P, F, T>(path: P, f: F) -> Result<Vec<T>>
where
    P: AsRef<Path>,
    F: Fn(&str) -> Result<T>,
{
    let reader = BufReader::new(File::open(path)?);
    let items = reader
        .lines()
        .map(|line| f(&line?))
        .collect::<Result<Vec<T>, _>>()?;

    Ok(items)
}

/// Parse a value from file contents.
///
/// This trait is most commonly used with types that implement [`FromStr`].
///
/// # Warning
///
/// The implementation for [`FromStr`] types assumes the entire file's contents
/// can be stored in memory with [`std::fs::read_to_string`]. Use caution when
/// calling `from_file` if there is potential for the input file to be
/// particularly large.
pub trait FromFile: Sized {
    type Err;

    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Err>
    where
        Self::Err: From<std::io::Error>;
}

impl<T: FromStr> FromFile for T
where
    <T as FromStr>::Err: From<std::io::Error>,
{
    type Err = <Self as FromStr>::Err;

    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Err> {
        let s = std::fs::read_to_string(path.as_ref())?;
        Self::from_str(&s)
    }
}
