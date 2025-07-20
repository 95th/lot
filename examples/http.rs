use std::time::Duration;

use anyhow::Result;
use lot::Executor;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let mut executor = Executor::new();
    executor.add_stage(0, 100, Duration::from_secs(10));
    executor.add_stage(100, 100, Duration::from_secs(10));
    executor.add_stage(100, 0, Duration::from_secs(10));

    let client = Client::new();
    executor.run(|| call_localhost(client.clone())).await;
}

async fn call_localhost(client: Client) -> Result<()> {
    let response = client.get("https://localhost:8080").send().await?;
    let response = response.error_for_status()?;
    response.bytes().await?;
    Ok(())
}
