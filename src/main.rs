use std::sync::{mpsc, Arc};

use clap::Parser;
use indicatif::ProgressBar;

use vanity_miner::{
    cli::{build_runtime_config, Args},
    core::{
        config::MiningConfig,
        executor::Executor,
        types::{FoundAddress, VanityError},
    },
    executors::CpuExecutor,
    output::{CombinedResultSink, ConsoleResultSink, ConsoleStatsSink, CsvResultSink, ResultSink},
    stats::{MiningStats, StatsReporter},
};

type Result<T> = std::result::Result<T, VanityError>;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    let runtime = build_runtime_config(&args)?;

    let stats = Arc::new(MiningStats::new());
    let executor = CpuExecutor::new(runtime.threads, Arc::clone(&stats));

    let (tx, rx) = mpsc::channel::<FoundAddress>();

    let progress = ProgressBar::new_spinner();

    let mut combined_sink = CombinedResultSink::default();

    if !runtime.no_file {
        let csv_sink = CsvResultSink::new(&runtime.output).map_err(VanityError::IoError)?;
        combined_sink.push(Box::new(csv_sink));
    }

    if runtime.console || runtime.no_file {
        combined_sink.push(Box::new(ConsoleResultSink::with_progress_bar(
            progress.clone(),
        )));
    }

    let mut sink = combined_sink;
    let sink_handle = std::thread::spawn(move || {
        for found in rx {
            if let Err(e) = sink.handle(&found) {
                eprintln!("Failed to write result: {e}");
            }
        }
    });

    let mut reporter = StatsReporter::new(Arc::clone(&stats));
    reporter.start(Box::new(ConsoleStatsSink::new(progress.clone())));

    let config = MiningConfig {
        chain: runtime.chain,
        matcher: runtime.matcher,
        generation_mode: runtime.generation_mode,
        address_config: runtime.address_config,
        max_attempts: runtime.max_attempts,
        limit: runtime.limit,
        result_sender: tx,
    };

    let result = executor.execute(config);

    reporter.stop();
    sink_handle
        .join()
        .expect("result sink thread should join successfully");

    println!("Done");
    println!("Found: {}", result.found);
    println!("Attempts: {}", result.attempts);
    println!("Hashrate: {:.2} addr/s", result.hashrate);

    Ok(())
}
