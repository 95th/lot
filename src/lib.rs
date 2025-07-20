//! # Lot - A simple load testing framework
//!
//! `lot` provides the basic building blocks for creating and running load tests in Rust.
//! It is designed to be simple, flexible, and easy to use.
//!
//! The main components of `lot` are:
//!
//! - **Scenario:** A scenario represents a single user's journey through your system.
//! - **Executor:** The executor is responsible for running the scenarios and collecting the results.

mod executor;
mod task;
mod timeline;

pub use executor::Executor;
pub use task::TaskFactory;
