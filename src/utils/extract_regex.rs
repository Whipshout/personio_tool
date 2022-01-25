use anyhow::{anyhow, Result};
use regex::Regex;

pub fn extract_with_regex(regex: Regex, body: &str) -> Result<&str> {
    let capture = if regex.is_match(body) {
        regex.captures(body).unwrap()
    } else {
        return Err(anyhow!("Could not find anything using: {}", regex));
    };

    Ok(capture.get(1).unwrap().as_str())
}
