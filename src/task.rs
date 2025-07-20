use anyhow::Result;
use std::future::Future;

/// Task factory.
///
/// It returns a "task" future representing one unit of work
/// that will be executed by the load generator.
pub trait TaskFactory {
    type Task: Future<Output = Result<()>> + Send + 'static;

    /// Creates a new task.
    fn create(&self) -> Self::Task;
}

impl<F, Task> TaskFactory for F
where
    F: Fn() -> Task,
    Task: Future<Output = Result<()>> + Send + 'static,
{
    type Task = Task;

    fn create(&self) -> Self::Task {
        self()
    }
}
