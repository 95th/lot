use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use anyhow::Result;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    time::Instant,
};

use crate::{scenario::Scenario, timeline::Timeline};

pub struct Executor {
    timelines: Vec<Timeline>,
    stop: AtomicBool,
    started: AtomicUsize,
    success: AtomicUsize,
    failure: AtomicUsize,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            timelines: Vec::new(),
            stop: AtomicBool::new(false),
            started: AtomicUsize::new(0),
            success: AtomicUsize::new(0),
            failure: AtomicUsize::new(0),
        }
    }

    pub fn add_stage(&mut self, start_rate: usize, end_rate: usize, duration: Duration) {
        self.timelines
            .push(Timeline::new(start_rate as f64, end_rate as f64, duration));
    }

    pub async fn run(&self, scenario: impl Scenario) {
        let (tx, rx) = unbounded_channel();
        let update_stats = self.update_stats(rx);
        let print_progress = self.print_progress();
        let run_scenario = self.run_scenario(scenario, tx);
        tokio::join!(update_stats, print_progress, run_scenario);
    }

    async fn run_scenario(
        &self,
        scenario: impl Scenario,
        result_channel: UnboundedSender<Result<()>>,
    ) {
        for (i, timeline) in self.timelines.iter().enumerate() {
            println!("Starting stage {i}");
            let start_time = Instant::now();
            for next_tick in timeline.into_iter() {
                tokio::time::sleep_until(start_time + next_tick).await;
                self.started.fetch_add(1, Ordering::AcqRel);
                let result_channel = result_channel.clone();
                let task = scenario.run();
                tokio::spawn(async move {
                    result_channel.send(task.await).unwrap();
                });
            }
            println!("Stage {i} completed");
        }
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
