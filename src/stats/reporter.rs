use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use super::types::{MiningStats, StatsSnapshot};

/// Sink that receives periodic stats snapshots.
pub trait StatsSink: Send {
    fn update(&mut self, current: &StatsSnapshot, previous: &StatsSnapshot);

    fn on_stop(&mut self, _final_snapshot: &StatsSnapshot) {}
}

/// Boxed stats sink for dynamic dispatch
pub type DynStatsSink = Box<dyn StatsSink>;

pub struct StatsReporter {
    stats: Arc<MiningStats>,
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    interval: Duration,
}

impl StatsReporter {
    pub fn new(stats: Arc<MiningStats>) -> Self {
        Self {
            stats,
            running: Arc::new(AtomicBool::new(true)),
            handle: None,
            interval: Duration::from_secs(1),
        }
    }

    pub fn with_interval(stats: Arc<MiningStats>, interval: Duration) -> Self {
        Self {
            stats,
            running: Arc::new(AtomicBool::new(true)),
            handle: None,
            interval,
        }
    }
    pub fn start(&mut self, sink: DynStatsSink) {
        let stats = Arc::clone(&self.stats);
        let running = Arc::clone(&self.running);
        let interval = self.interval;

        let handle = thread::spawn(move || {
            let mut sink = sink;
            let mut last_snapshot = stats.get_snapshot();

            while running.load(Ordering::Relaxed) {
                thread::sleep(interval);

                let current_snapshot = stats.get_snapshot();
                sink.update(&current_snapshot, &last_snapshot);
                last_snapshot = current_snapshot;
            }

            let final_snapshot = stats.get_snapshot();
            sink.on_stop(&final_snapshot);
        });

        self.handle = Some(handle);
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct CollectingSink {
        snapshots: Arc<Mutex<Vec<StatsSnapshot>>>,
    }

    impl CollectingSink {
        fn new(shared: Arc<Mutex<Vec<StatsSnapshot>>>) -> Self {
            Self { snapshots: shared }
        }
    }

    impl StatsSink for CollectingSink {
        fn update(&mut self, current: &StatsSnapshot, _previous: &StatsSnapshot) {
            self.snapshots.lock().unwrap().push(current.clone());
        }
    }

    #[test]
    fn reporter_calls_sink_periodically() {
        let stats = Arc::new(MiningStats::new());
        let collected = Arc::new(Mutex::new(Vec::new()));

        let sink: DynStatsSink = Box::new(CollectingSink::new(collected.clone()));

        let mut reporter = StatsReporter::with_interval(stats.clone(), Duration::from_millis(50));
        reporter.start(sink);

        stats.add_attempts(100);

        std::thread::sleep(Duration::from_millis(200));
        reporter.stop();

        let snapshots = collected.lock().unwrap();
        assert!(!snapshots.is_empty());
        assert!(snapshots.last().unwrap().attempts >= 100);
    }
}
