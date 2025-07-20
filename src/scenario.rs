use anyhow::Result;
use std::future::Future;

/// A scenario represents a single user's journey through your system.
///
/// It is a single unit of work that will be executed by the load generator.
/// Scenarios are defined as async functions that return a `Result<()>`.
pub trait Scenario {
    /// Runs the scenario.
    fn run(&self) -> impl Future<Output = Result<()>> + Send + 'static;
}

impl<F, Fut> Scenario for F
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    fn run(&self) -> impl Future<Output = Result<()>> + Send + 'static {
        self()
    }
}
