use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use anyhow::Result;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    time::Instant,
};

use crate::task::TaskFactory;
use crate::timeline::Timeline;

/// The `Executor` is responsible for running the scenarios and collecting the results.
///
/// It manages the timeline of the load test, spinning up new scenarios as required.
/// It also collects statistics about the test run, such as the number of started,
/// successful, and failed scenarios.
pub struct Executor {
    timelines: Vec<Timeline>,
    stop: AtomicBool,
    started: AtomicUsize,
    success: AtomicUsize,
    failure: AtomicUsize,
}

impl Executor {
    /// Creates a new `Executor`.
    pub fn new() -> Self {
        Self {
            timelines: Vec::new(),
            stop: AtomicBool::new(false),
            started: AtomicUsize::new(0),
            success: AtomicUsize::new(0),
            failure: AtomicUsize::new(0),
        }
    }

    /// Adds a new stage to the load test.
    ///
    /// A stage is a period of time during which the load is ramped up from a starting
    /// rate to an ending rate.
    ///
    /// # Arguments
    ///
    /// * `start_rate` - The number of scenarios to start per second at the beginning of the stage.
    /// * `end_rate` - The number of scenarios to start per second at the end of the stage.
    /// * `duration` - The duration of the stage.
    pub fn add_stage(&mut self, start_rate: usize, end_rate: usize, duration: Duration) {
        self.timelines
            .push(Timeline::new(start_rate as f64, end_rate as f64, duration));
    }

    /// Runs the load test.
    ///
    /// This will execute all the configured stages in order.
    ///
    /// # Arguments
    ///
    /// * `scenario` - The scenario to run.
    pub async fn run(&self, task_factory: impl TaskFactory) {
        let (tx, rx) = unbounded_channel();
        let update_stats = self.update_stats(rx);
        let print_progress = self.print_progress();
        let run_scenario = self.run_scenario(task_factory, tx);
        tokio::join!(update_stats, print_progress, run_scenario);
    }

    async fn run_scenario(
        &self,
        task_factory: impl TaskFactory,
        result_channel: UnboundedSender<Result<()>>,
    ) {
        for (i, timeline) in self.timelines.iter().enumerate() {
            println!("Starting stage {i}");
            let start_time = Instant::now();
            for next_tick in timeline.into_iter() {
                tokio::time::sleep_until(start_time + next_tick).await;
                self.started.fetch_add(1, Ordering::AcqRel);
                let result_channel = result_channel.clone();
                let task = task_factory.create();
                tokio::spawn(async move {
                    result_channel.send(task.await).unwrap();
                });
            }
            println!("Stage {i} completed");
        }
        // Once all stages are complete, the sender is dropped, which will cause the
        // `update_stats` task to complete.
    }

    async fn update_stats(&self, mut result_channel: UnboundedReceiver<Result<()>>) {
        while let Some(result) = result_channel.recv().await {
            match result {
                Ok(_) => self.success.fetch_add(1, Ordering::AcqRel),
                Err(_) => self.failure.fetch_add(1, Ordering::AcqRel),
            };
        }
        self.stop.store(true, Ordering::Release);
    }

    async fn print_progress(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        while !self.stop.load(Ordering::Acquire) {
            interval.tick().await;
            let started = self.started.load(Ordering::Acquire);
            let success = self.success.load(Ordering::Acquire);
            let failure = self.failure.load(Ordering::Acquire);
            println!("Started: {started:>10}, Success: {success:>10}, Failure: {failure:>10}");
        }
    }
}
