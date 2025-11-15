use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

static RELAXED: Ordering = Ordering::Relaxed;

/// Thread-safe counters for mining run.
///
/// The struct itself is intended to be wrapped in `Arc<MiningStats>`
#[derive(Debug)]
pub struct MiningStats {
    start_time: Instant,
    /// Total number of attempts made so far.
    pub attempt_count: AtomicU64,
    /// Total number of matching addresses found so far.
    pub found_count: AtomicU64,
}

impl MiningStats {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            attempt_count: AtomicU64::new(0),
            found_count: AtomicU64::new(0),
        }
    }

    pub fn start_time(&self) -> Instant {
        self.start_time
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn attempts(&self) -> u64 {
        self.attempt_count.load(RELAXED)
    }

    pub fn found(&self) -> u64 {
        self.found_count.load(RELAXED)
    }

    pub fn increment_attempt(&self) {
        self.add_attempts(1);
    }

    pub fn increment_found(&self) {
        self.add_found(1);
    }

    pub fn add_attempts(&self, n: u64) {
        if n > 0 {
            self.attempt_count.fetch_add(n, RELAXED);
        }
    }

    pub fn add_found(&self, n: u64) {
        if n > 0 {
            self.found_count.fetch_add(n, RELAXED);
        }
    }

    pub fn get_snapshot(&self) -> StatsSnapshot {
        StatsSnapshot {
            attempts: self.attempts(),
            found: self.found(),
            timestamp: Instant::now(),
            elapsed: self.elapsed(),
        }
    }
}

impl Default for MiningStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Immutable snapshot of mining stats at a given moment.
#[derive(Debug, Clone)]
pub struct StatsSnapshot {
    pub attempts: u64,
    pub found: u64,
    /// Wall-clock time when the snapshot was taken.
    pub timestamp: Instant,
    /// Elapsed time since the corresponding `MiningStats` was created.
    pub elapsed: Duration,
}

impl StatsSnapshot {
    pub fn calculate_speed(&self, previous: &StatsSnapshot) -> u64 {
        let elapsed = self.timestamp.duration_since(previous.timestamp);
        let secs = elapsed.as_secs_f64();
        if secs == 0.0 {
            return 0;
        }

        let attempts_delta = self.attempts.saturating_sub(previous.attempts) as f64;
        (attempts_delta / secs).round() as u64
    }

    pub fn hashrate(&self) -> f64 {
        let secs = self.elapsed.as_secs_f64();
        if secs == 0.0 {
            0.0
        } else {
            self.attempts as f64 / secs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn calculate_speed_zero_elapsed_returns_zero() {
        let t = Instant::now();

        let previous = StatsSnapshot {
            attempts: 100,
            found: 1,
            timestamp: t,
            elapsed: Duration::from_secs(1),
        };

        // Same timestamp -> elapsed_secs = 0
        let current = StatsSnapshot {
            attempts: 200,
            found: 2,
            timestamp: t,
            elapsed: Duration::from_secs(2),
        };

        assert_eq!(current.calculate_speed(&previous), 0);
    }

    #[test]
    fn calculate_speed_normal_case() {
        let t = Instant::now();

        let previous = StatsSnapshot {
            attempts: 100,
            found: 1,
            timestamp: t,
            elapsed: Duration::from_secs(1),
        };

        let current = StatsSnapshot {
            attempts: 300,
            found: 2,
            timestamp: t + Duration::from_secs(2),
            elapsed: Duration::from_secs(3),
        };

        // Attempts increased by 200 over 2 seconds -> 100 addr/s
        assert_eq!(current.calculate_speed(&previous), 100);
    }

    #[test]
    fn hashrate_uses_elapsed_time() {
        let snapshot = StatsSnapshot {
            attempts: 200,
            found: 0,
            timestamp: Instant::now(),
            elapsed: Duration::from_secs(4),
        };

        let rate = snapshot.hashrate();
        assert!((rate - 50.0).abs() < 1e-6);
    }

    #[test]
    fn mining_stats_helpers_update_counters() {
        let stats = MiningStats::new();

        stats.increment_attempt();
        stats.add_attempts(9);
        stats.increment_found();
        stats.add_found(4);

        let snapshot = stats.get_snapshot();
        assert_eq!(snapshot.attempts, 10);
        assert_eq!(snapshot.found, 5);
    }
}
