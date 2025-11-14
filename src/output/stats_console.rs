use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::stats::{StatsSink, StatsSnapshot};

/// Console-based stats sink using `indicatif` spinners
#[derive(Debug)]
pub struct ConsoleStatsSink {
    multi: MultiProgress,
    speed_pb: ProgressBar,
    total_pb: ProgressBar,
}

impl ConsoleStatsSink {
    pub fn new() -> Self {
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

        Self {
            multi,
            speed_pb,
            total_pb,
        }
    }
}

impl StatsSink for ConsoleStatsSink {
    fn update(&mut self, current: &StatsSnapshot, previous: &StatsSnapshot) {
        let speed = current.calculate_speed(previous);
        self.speed_pb
            .set_message(format!("Speed: {} addresses/s", speed));

        self.total_pb.set_message(format!(
            "Total attempts: {}, Total found: {}",
            current.attempts, current.found
        ));
    }

    fn on_stop(&mut self, final_snapshot: &StatsSnapshot) {
        self.speed_pb.finish_with_message("Mining stopped");
        self.total_pb.finish_with_message(format!(
            "Total attempts: {}, Total found: {}",
            final_snapshot.attempts, final_snapshot.found
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::MiningStats;
    use std::sync::Arc;

    #[test]
    fn console_stats_sink_handles_updates() {
        let stats = Arc::new(MiningStats::new());
        let mut sink = ConsoleStatsSink::new();

        let prev = stats.get_snapshot();
        stats.add_attempts(100);
        let current = stats.get_snapshot();

        sink.update(&current, &prev);
        sink.on_stop(&current);
    }
}
