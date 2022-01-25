use anyhow::Result;
use owo_colors::OwoColorize;

use personio_tool::startup::{initialize, run};

#[tokio::main]
async fn main() -> Result<()> {
    println!(
        "{}",
        "--------------------------------------------------------"
            .on_blue()
            .black()
    );

    let (config, urls, client) = initialize("config.json")?;

    run(config, urls, client).await?;

    Ok(())
}
