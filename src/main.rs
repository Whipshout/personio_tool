use anyhow::Result;

use personio_tool::startup::{initialize, run};

#[tokio::main]
async fn main() -> Result<()> {
    let (config, urls, logger, client) = initialize("config.json")?;

    run(config, urls, logger, client).await?;

    Ok(())
}
