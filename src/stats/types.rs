use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};

static RELAXED: Ordering = Ordering::Relaxed;

#[derive(Debug)]
pub struct MiningStats {
    pub attempt_count: Arc<AtomicU64>,
    pub found_count: Arc<AtomicU64>,
}

impl MiningStats {
    pub fn new() -> Self {
        Self {
            attempt_count: Arc::new(AtomicU64::new(0)),
            found_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn increment_attempt(&self) {
        self.attempt_count.fetch_add(1, RELAXED);
    }

    pub fn increment_found(&self) {
        self.found_count.fetch_add(1, RELAXED);
    }

    pub fn get_snapshot(&self) -> StatsSnapshot {
        StatsSnapshot {
            attempts: self.attempt_count.load(RELAXED),
            found: self.found_count.load(RELAXED),
            timestamp: Instant::now(),
        }
    }
}

impl Default for MiningStats {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct StatsSnapshot {
    pub attempts: u64,
    pub found: u64,
    pub timestamp: Instant,
}

impl StatsSnapshot {
    pub fn calculate_speed(&self, previous: &StatsSnapshot) -> u64 {
        let elapsed = self.timestamp.duration_since(previous.timestamp).as_secs();
        if elapsed == 0 {
            return 0;
        }
        (self.attempts - previous.attempts) / elapsed
    }
} 