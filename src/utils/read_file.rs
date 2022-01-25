use std::fs::File;
use std::io::Read;

use anyhow::{Context, Result};

pub fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path).with_context(|| format!("Failed to open file: {}", path))?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)
        .with_context(|| format!("Unable to read: {}", path))?;

    Ok(buff)
}
