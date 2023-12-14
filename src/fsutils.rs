use std::{fs::File, io::BufRead, io::BufReader, path::Path};

use anyhow::Result;

pub fn buf_reader<P: AsRef<Path>>(path: P) -> Result<BufReader<File>> {
    let file = File::open(path)?;

    Ok(BufReader::new(file))
}

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
