use std::time::Duration;

use anyhow::Result;
use lot::Executor;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut executor = Executor::new();
    executor.add_stage(0, 100, Duration::from_secs(10));
    executor.add_stage(100, 100, Duration::from_secs(10));
    executor.add_stage(100, 0, Duration::from_secs(10));
    executor.run(my_scenario).await;
}

async fn my_scenario() -> Result<()> {
    // Add your load test logic here
    Ok(())
}
