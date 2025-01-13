mod address;
mod address_generator;
mod cli;
mod validator;

use address_generator::AddressGenerator;
use clap::Parser;
use num_cpus;

use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc, Arc,
    },
    thread,
    time::Duration,
};

static RELAXED: Ordering = Ordering::Relaxed;

fn main() {
    let args = cli::Args::parse();
    let num_threads = args.threads.unwrap_or(num_cpus::get());
    let mut handles = vec![];

    let max_attempts = args.max_attempts.unwrap_or(0);
    let limit = args.limit.unwrap_or(0);
    let found_count = Arc::new(AtomicU64::new(0));
    let attempt_count = Arc::new(AtomicU64::new(0));

    let (tx, rx) = mpsc::channel();

    let attempt_count_clone = attempt_count.clone();
    let found_count_clone = found_count.clone();

    let status_thread_running = Arc::new(AtomicBool::new(true));

    let result_handle = thread::spawn(move || {
        for (addr, pk) in rx {
            println!("found address: {}, secret: {}", addr, pk);
        }
    });

    let status_thread_running_clone = status_thread_running.clone();
    let stats_handle = thread::spawn(move || {
        let mut last_attempt = 0u64;
        let mut elapsed_time = Duration::from_secs(0);

        loop {
            thread::sleep(Duration::from_secs(1));
            elapsed_time += Duration::from_secs(1);

            if !status_thread_running_clone.load(RELAXED) {
                break;
            }

            let current_attempt = attempt_count_clone.load(RELAXED);
            let current_found = found_count_clone.load(RELAXED);

            let attempts_per_second =
                (current_attempt - last_attempt) as f64 / elapsed_time.as_secs_f64();

            println!(
                "Speed: {:} addresses/s, Total attempts: {}, Total found: {}",
                attempts_per_second, current_attempt, current_found
            );
            last_attempt = current_attempt;
            elapsed_time = Duration::from_secs(0)
        }
    });

    for _ in 0..num_threads {
        let tx_clone = tx.clone();

        let found_count_clone = found_count.clone();
        let attempt_count_clone = attempt_count.clone();

        let derivation_path = args.derivation_path.clone();
        let from_private_key = args.from_private_key.clone();
        let address_format = args.address_format.clone();
        let network = args.cfx_network.clone();
        let validator = validator::AddressValidator::new(
            args.contains.clone(),
            args.prefix.clone(),
            args.suffix.clone(),
            args.regex.clone(),
        );

        let handle = thread::spawn(move || {
            let address_generator =
                AddressGenerator::new(from_private_key, derivation_path, validator, address_format);

            loop {
                if (max_attempts > 0 && attempt_count_clone.load(RELAXED) >= max_attempts)
                    || (limit > 0 && found_count_clone.load(RELAXED) >= limit)
                {
                    break;
                }

                if let Some((addr, secret)) = address_generator.new_random_address(network) {
                    tx_clone.send((addr, secret)).unwrap()
                }

                attempt_count_clone.fetch_add(1, RELAXED);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    drop(tx);
    status_thread_running.store(false, RELAXED);

    stats_handle.join().unwrap();
    result_handle.join().unwrap();

    println!("Done");
    println!("Found: {}", found_count.load(RELAXED));
    println!("Attempts: {}", attempt_count.load(RELAXED));
}
