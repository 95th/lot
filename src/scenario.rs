use anyhow::Result;

pub trait Scenario {
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
