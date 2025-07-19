use std::time::Duration;

use anyhow::Result;
use lot::executor::Executor;
use lot::scenario::Scenario;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let mut executor = Executor::new();
    executor.add_stage(0, 100, Duration::from_secs(10));
    executor.add_stage(100, 100, Duration::from_secs(10));
    executor.add_stage(100, 0, Duration::from_secs(10));
    executor.run(CallLocalhost::new()).await;
}

struct CallLocalhost {
    client: Client,
}

impl CallLocalhost {
    fn new() -> Self {
        Self {
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        }
    }
}

impl Scenario for CallLocalhost {
    fn run(&self) -> impl Future<Output = Result<()>> + Send + 'static {
        let client = self.client.clone();
        async move {
            let response = client.get("https://localhost:8080").send().await?;
            let response = response.error_for_status()?;
            response.bytes().await?;
            Ok(())
        }
    }
}
