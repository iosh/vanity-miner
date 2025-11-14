use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
    sync::{atomic::Ordering, mpsc, Arc},
    thread,
};

use rayon::prelude::*;

mod address;
mod cli;
mod generator;
mod validator;

use clap::Parser;
use csv::Writer;
use generator::AddressGenerator;
use num_cpus;
use vanity_miner::output::ConsoleStatsSink;
use vanity_miner::stats::{MiningStats, StatsReporter};

// Threshold for updating global atomic counters to reduce contention
const LOCAL_COUNTER_THRESHOLD: u64 = 1000;

static RELAXED: Ordering = Ordering::Relaxed;

// Initialize CSV writer with proper error handling
fn init_csv_writer(path: &Path) -> io::Result<Writer<File>> {
    // Check if file exists and has content
    let file_exists = path.exists() && path.metadata()?.len() > 0;

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;

    let mut writer = Writer::from_writer(file);

    // Only write header if file is new or empty
    if !file_exists {
        writer.write_record(&["address", "secret"])?;
        writer.flush()?;
    }

    Ok(writer)
}

fn main() {
    let args = cli::Args::parse();
    let num_threads = args.threads.unwrap_or(num_cpus::get());

    // Initialize rayon's thread pool with specified number of threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let max_attempts = args.max_attempts.unwrap_or(0);
    let limit = args.limit.unwrap_or(0);
    let stats = Arc::new(MiningStats::new());

    let (tx, rx) = mpsc::channel();

    let output_path = Path::new(&args.output_file);

    // Initialize CSV writer with error handling
    let mut csv_writer = init_csv_writer(output_path).unwrap_or_else(|e| {
        eprintln!("Failed to initialize CSV writer: {}", e);
        std::process::exit(1);
    });

    // Result writer thread with improved error handling
    let result_handle = thread::spawn(move || {
        for (addr, secret) in rx {
            if let Err(e) = csv_writer.write_record(&[&addr, &secret]) {
                eprintln!("Failed to write record: {}", e);
                continue;
            }
            if let Err(e) = csv_writer.flush() {
                eprintln!("Failed to flush writer: {}", e);
            }
        }
    });

    let mut stats_reporter = StatsReporter::new(stats.clone());
    let stats_sink = ConsoleStatsSink::new();
    stats_reporter.start(Box::new(stats_sink));

    let worker_configs: Vec<_> = (0..num_threads)
        .map(|_| {
            let tx = tx.clone();
            let stats = stats.clone();
            let derivation_path = args.derivation_path.clone();
            let from_private_key = args.use_private_key;
            let address_format = args.address_format.clone();
            let network = args.cfx_network.clone();

            let validator = {
                let mut builder = validator::ValidatorBuilder::new();
                if let Some(contains) = args.contains.clone() {
                    builder = builder.with_contains(contains);
                }
                if let Some(prefix) = args.prefix.clone() {
                    builder = builder.with_prefix(prefix);
                }
                if let Some(suffix) = args.suffix.clone() {
                    builder = builder.with_suffix(suffix);
                }
                if let Some(regex) = args.regex.clone() {
                    builder = builder.with_regex(regex);
                }
                builder.build()
            };

            (
                tx,
                stats,
                from_private_key,
                derivation_path,
                address_format,
                network,
                validator,
            )
        })
        .collect();

    worker_configs.into_par_iter().for_each(
        |(tx, stats, from_private_key, derivation_path, address_format, network, validator)| {
            let address_generator = if from_private_key {
                AddressGenerator::private_key()
            } else {
                AddressGenerator::mnemonic(derivation_path)
            }
            .with_format(address_format)
            .with_validator(validator)
            .build();

            let mut local_attempt_count = 0u64;
            let mut local_found_count = 0u64;

            loop {
                // Check global limits using relaxed ordering
                let global_attempts = stats.attempt_count.load(RELAXED);
                let global_found = stats.found_count.load(RELAXED);

                if (max_attempts > 0 && global_attempts + local_attempt_count >= max_attempts)
                    || (limit > 0 && global_found + local_found_count >= limit)
                {
                    // Update global counters before exit
                    if local_attempt_count > 0 {
                        stats.attempt_count.fetch_add(local_attempt_count, RELAXED);
                    }
                    if local_found_count > 0 {
                        stats.found_count.fetch_add(local_found_count, RELAXED);
                    }
                    break;
                }

                if let Some((addr, secret)) = address_generator.new_random_address(network) {
                    // Send result immediately since finding addresses is rare
                    tx.send((addr, secret)).unwrap();
                    local_found_count += 1;

                    // Update global found counter immediately for accurate progress tracking
                    if local_found_count > 0 {
                        stats.found_count.fetch_add(local_found_count, RELAXED);
                        local_found_count = 0;
                    }
                }

                local_attempt_count += 1;
                if local_attempt_count >= LOCAL_COUNTER_THRESHOLD {
                    stats.attempt_count.fetch_add(local_attempt_count, RELAXED);
                    local_attempt_count = 0;
                }
            }
        },
    );

    drop(tx);

    let final_stats = stats.get_snapshot();
    stats_reporter.stop();
    result_handle.join().unwrap();

    println!("Done");
    println!("Found: {}", final_stats.found);
    println!("Attempts: {}", final_stats.attempts);
}
