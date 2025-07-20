# Lot

A simple, async-native load testing framework for Rust.

`lot` is a load testing framework built on top of `tokio` that allows you to define and run scenarios against your services. It's designed to be lightweight and easy to use, while still providing the flexibility to model complex user behaviors.

## Getting Started

To start using `lot`, add it to your `Cargo.toml`:

```toml
[dependencies]
lot = "0.1"
```

Then, you can create a test scenario and run it using the `Executor`.

## Example

Here's an example of a simple load test that makes HTTP requests to a local server:

```rust
use std::time::Duration;
use anyhow::Result;
use lot::executor::Executor;
use lot::scenario::Scenario;
use reqwest::Client;
use std::future::Future;

// 1. Define your scenario
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

// 2. Configure and run the executor
#[tokio::main]
async fn main() {
    let mut executor = Executor::new();

    // Ramp up from 0 to 100 rps over 10 seconds
    executor.add_stage(0, 100, Duration::from_secs(10));
    // Stay at 100 rps for 10 seconds
    executor.add_stage(100, 100, Duration::from_secs(10));
    // Ramp down from 100 to 0 rps over 10 seconds
    executor.add_stage(100, 0, Duration::from_secs(10));

    executor.run(CallLocalhost::new()).await;
}

This example demonstrates a multi-stage load test:
1.  A ramp-up phase.
2.  A sustained load phase.
3.  A ramp-down phase.

## Contributing

Contributions are welcome! If you have a feature request, bug report, or pull request, please feel free to open an issue.

## License

This project is licensed under the terms of the [LICENSE](LICENSE) file.
```
