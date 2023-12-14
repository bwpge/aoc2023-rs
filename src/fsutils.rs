//! Additional utilities for files, I/O, etc.

use std::{fs::File, io::BufRead, io::BufReader, path::Path};

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
