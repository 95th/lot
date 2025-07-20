use std::time::Duration;

/// `Timeline` defines how many users are running at any given time.
///
/// It is an iterator that yields the next time a scenario should be started.
/// The `Timeline` can be configured to have a constant or a ramping arrival rate.
#[derive(Debug, Clone, Copy)]
pub struct Timeline {
    start_rate: f64,
    end_rate: f64,
    duration: f64,
    total_iterations: u64,
    current_iteration: u64,
}

impl Timeline {
    /// Creates a new `Timeline`.
    ///
    /// # Arguments
    ///
    /// * `start_rate` - The number of scenarios to start per second at the beginning of the timeline.
    /// * `end_rate` - The number of scenarios to start per second at the end of the timeline.
    /// * `duration` - The duration of the timeline.
    pub fn new(start_rate: f64, end_rate: f64, duration: Duration) -> Self {
        let total_iterations = if start_rate == end_rate {
            duration.as_secs_f64() * end_rate
        } else {
            duration.as_secs_f64() * (start_rate + end_rate) / 2.0
        };

        Self {
            start_rate,
            end_rate,
            duration: duration.as_secs_f64(),
            total_iterations: total_iterations.ceil() as u64,
            current_iteration: 1,
        }
    }
}

impl Iterator for Timeline {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_iteration > self.total_iterations {
            return None;
        }

        let i = self.current_iteration as f64;
        let time_offset_secs = if self.start_rate == self.end_rate {
            (i - 1.0) / self.end_rate
        } else {
            let from = self.start_rate;
            let to = self.end_rate;

            // This formula is derived from the integral of the rate function.
            // The rate function is a linear ramp from `start_rate` to `end_rate`.
            // We want to find the time `t` at which the number of started scenarios (the area
            // under the curve) is equal to `i`.
            //
            // The rate function is `r(t) = start_rate + (end_rate - start_rate) * t / duration`.
            // The number of started scenarios is `n(t) = integral(r(t) dt) = start_rate * t + (end_rate - start_rate) * t^2 / (2 * duration)`.
            // We want to solve `n(t) = i` for `t`. This is a quadratic equation, and the
            // solution is the formula below.
            let discriminant =
                from * from * self.duration * self.duration + 2.0 * i * (to - from) * self.duration;
            let numerator = -from * self.duration + discriminant.max(0.0).sqrt();
            let denominator = to - from;

            numerator / denominator
        };

        self.current_iteration += 1;
        Some(Duration::from_secs_f64(time_offset_secs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ramping_arrival_rate() {
        let timeline = Timeline::new(100.0, 1.0, Duration::from_secs(1000));
        for next_tick in timeline {
            println!("duration: {:?}", next_tick);
        }
    }
}
