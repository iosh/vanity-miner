use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

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
            let multi = MultiProgress::new();
            
            let speed_pb = multi.add(ProgressBar::new_spinner());
            speed_pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} [{elapsed_precise}] {msg}")
                    .unwrap(),
            );

            let total_pb = multi.add(ProgressBar::new_spinner());
            total_pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.blue} {msg}")
                    .unwrap(),
            );

            let mut last_snapshot = stats.get_snapshot();

            while running.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(1));

                let current_snapshot = stats.get_snapshot();
                let speed = current_snapshot.calculate_speed(&last_snapshot);

                speed_pb.set_message(format!("Speed: {} addresses/s", speed));
                total_pb.set_message(format!(
                    "Total attempts: {}, Total found: {}",
                    current_snapshot.attempts, current_snapshot.found
                ));

                last_snapshot = current_snapshot;
            }

            speed_pb.finish_with_message("Mining stopped");
            total_pb.finish();
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
