mod address;
mod cli;
mod generator;
mod stats;
mod validator;

use clap::Parser;
use csv::Writer;
use generator::AddressGenerator;
use num_cpus;
use stats::MiningStats;

use std::{
    path::Path,
    sync::{atomic::Ordering, mpsc, Arc},
    thread,
};

static RELAXED: Ordering = Ordering::Relaxed;

fn main() {
    let args = cli::Args::parse();
    let num_threads = args.threads.unwrap_or(num_cpus::get());
    let mut handles = vec![];

    let max_attempts = args.max_attempts.unwrap_or(0);
    let limit = args.limit.unwrap_or(0);
    let stats = Arc::new(MiningStats::new());

    let (tx, rx) = mpsc::channel();

    let output_path = Path::new(&args.output_file);

    let mut csv_writer = Writer::from_path(&output_path).expect("create csv file failed");

    csv_writer
        .write_record(&["address", "secret"])
        .expect("write csv header failed");

    let result_handle = thread::spawn(move || {
        for (addr, secret) in rx {
            csv_writer
                .write_record(&[&addr, &secret])
                .expect("write csv record failed");
            csv_writer.flush().expect("flush csv writer failed");
        }
    });

    let mut stats_reporter = stats::StatsReporter::new(stats.clone());
    stats_reporter.start();

    for _ in 0..num_threads {
        let tx_clone = tx.clone();
        let stats = stats.clone();

        let derivation_path = args.derivation_path.clone();
        let from_private_key = args.use_private_key;
        let address_format = args.address_format.clone();
        let network = args.cfx_network.clone();

        let validator = validator::ValidatorBuilder::new();
        let validator = if let Some(contains) = args.contains.clone() {
            validator.with_contains(contains)
        } else {
            validator
        };

        let validator = if let Some(prefix) = args.prefix.clone() {
            validator.with_prefix(prefix)
        } else {
            validator
        };

        let validator = if let Some(suffix) = args.suffix.clone() {
            validator.with_suffix(suffix)
        } else {
            validator
        };

        let validator = if let Some(regex) = args.regex.clone() {
            validator.with_regex(regex)
        } else {
            validator
        };

        let validator = validator.build();

        let handle = thread::spawn(move || {
            let address_generator = if from_private_key {
                AddressGenerator::private_key()
            } else {
                AddressGenerator::mnemonic(derivation_path)
            }
            .with_format(address_format)
            .with_validator(validator)
            .build();

            loop {
                if (max_attempts > 0 && stats.attempt_count.load(RELAXED) >= max_attempts)
                    || (limit > 0 && stats.found_count.load(RELAXED) >= limit)
                {
                    break;
                }

                if let Some((addr, secret)) = address_generator.new_random_address(network) {
                    tx_clone.send((addr, secret)).unwrap();
                    stats.increment_found();
                }

                stats.increment_attempt();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    drop(tx);

    let final_stats = stats.get_snapshot();
    stats_reporter.stop();
    result_handle.join().unwrap();

    println!("Done");
    println!("Found: {}", final_stats.found);
    println!("Attempts: {}", final_stats.attempts);
}
