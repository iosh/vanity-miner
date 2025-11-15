use indicatif::{ProgressBar, ProgressStyle};

use crate::stats::{StatsSink, StatsSnapshot};

/// Console-based stats sink using `indicatif` spinners
#[derive(Debug)]
pub struct ConsoleStatsSink {
    progress: ProgressBar,
}

impl ConsoleStatsSink {
    pub fn new(progress: ProgressBar) -> Self {
        progress.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );

        Self { progress }
    }
}

impl StatsSink for ConsoleStatsSink {
    fn update(&mut self, current: &StatsSnapshot, previous: &StatsSnapshot) {
        let speed = current.calculate_speed(previous);
        self.progress.set_message(format!(
            "Speed: {} addresses/s | Total attempts: {} | Total found: {}",
            speed, current.attempts, current.found
        ));
    }

    fn on_stop(&mut self, final_snapshot: &StatsSnapshot) {
        self.progress.finish_with_message(format!(
            "Mining stopped. Total attempts: {}, Total found: {}",
            final_snapshot.attempts, final_snapshot.found
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::MiningStats;
    use indicatif::ProgressBar;
    use std::sync::Arc;

    #[test]
    fn console_stats_sink_handles_updates() {
        let stats = Arc::new(MiningStats::new());
        let pb = ProgressBar::hidden();
        let mut sink = ConsoleStatsSink::new(pb);

        let prev = stats.get_snapshot();
        stats.add_attempts(100);
        let current = stats.get_snapshot();

        sink.update(&current, &prev);
        sink.on_stop(&current);
    }
}
