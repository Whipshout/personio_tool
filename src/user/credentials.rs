use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

impl Credentials {
    pub async fn login(&self, client: &Client, url: &str) -> Result<String> {
        let response_login = client
            .post(url)
            .form(self)
            .send()
            .await?
            .text()
            .await
            .with_context(|| "Login request failed")?;

        if response_login.contains("This page is currently not available") {
            return Err(anyhow!("Web down or connexion blocked"));
        }
        if !response_login.contains("employeeName") {
            return Err(anyhow!("Invalid credentials"));
        }

        Ok(response_login)
    }
}
