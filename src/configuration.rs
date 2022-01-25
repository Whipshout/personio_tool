use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::settings::{Dates, Params, Times};
use crate::user::Credentials;
use crate::utils::read_file;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub credentials: Credentials,
    pub times: Times,
    pub dates: Dates,
    pub params: Params,
}

pub fn get_configuration(path: &str) -> Result<Configuration> {
    let config = read_file(path)?;

    Ok(serde_json::from_str(&config)
        .with_context(|| format!("Unable to convert file {} to json", path))
        .unwrap())
}
