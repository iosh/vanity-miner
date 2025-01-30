use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use super::types::MiningStats;

pub struct StatsReporter {
    stats: Arc<MiningStats>,
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl StatsReporter {
    pub fn new(stats: Arc<MiningStats>) -> Self {
        Self {
            stats,
            running: Arc::new(AtomicBool::new(true)),
            handle: None,
        }
    }

    pub fn start(&mut self) {
        let stats = self.stats.clone();
        let running = self.running.clone();

        let handle = thread::spawn(move || {
            let mut last_snapshot = stats.get_snapshot();

            while running.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(1));

                let current_snapshot = stats.get_snapshot();
                let speed = current_snapshot.calculate_speed(&last_snapshot);

                println!(
                    "Speed: {} addresses/s, Total attempts: {}, Total found: {}",
                    speed, current_snapshot.attempts, current_snapshot.found
                );

                last_snapshot = current_snapshot;
            }
        });

        self.handle = Some(handle);
    }

    pub fn stop(mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}
