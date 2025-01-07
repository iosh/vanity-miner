mod address;
mod args;
mod validator;
use address::PrivateKeyAccount;

use clap::Parser;
use num_cpus;
use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc, Arc,
    },
    thread,
    time::{Duration, Instant},
};

static RELAXED: Ordering = Ordering::Relaxed;

fn main() {
    let args = args::Args::parse();
    let num_threads = args.threads.unwrap_or(num_cpus::get());
    let mut handles = vec![];
    
    let max = args.max.unwrap_or(0);
    let limit = args.limit.unwrap_or(0);
    let validator = Arc::new(validator::AddressValidator::new(&args));
    let found_count = Arc::new(AtomicU64::new(0));
    let attempt_count = Arc::new(AtomicU64::new(0));
    let (tx, rx) = mpsc::channel();

    let attempt_count_clone = attempt_count.clone();
    let found_count_clone = found_count.clone();

    let status_thread_running = Arc::new(AtomicBool::new(true));

    let status_thread_running_clone = status_thread_running.clone();
    let stats_handle = thread::spawn(move || {
        let mut last_attempt = 0u64;
        let mut last_time = Instant::now();

        loop {
            if !status_thread_running_clone.load(RELAXED) {
                break;
            }
            thread::sleep(Duration::from_secs(1));
            let current_time = Instant::now();
            let current_attempt = attempt_count_clone.load(RELAXED);
            let current_found = found_count_clone.load(RELAXED);

            let attempts_per_second = current_attempt - last_attempt;
            let elapsed = current_time.duration_since(last_time).as_secs_f64();

            println!(
                "Speed: {:.2} addresses/s, Total attempts: {}, Total found: {}",
                attempts_per_second as f64 / elapsed,
                current_attempt,
                current_found
            );

            last_attempt = current_attempt;
            last_time = current_time;
        }
    });

    for _ in 0..num_threads {
        let tx_clone = tx.clone();
        let validator_clone = validator.clone();
        let found_count_clone = found_count.clone();
        let attempt_count_clone = attempt_count.clone();

        let handle = thread::spawn(move || loop {
            if (max > 0 && attempt_count_clone.load(RELAXED) >= max)
                || (limit > 0 && found_count_clone.load(RELAXED) >= limit)
            {
                break;
            }

            let private_key_account = PrivateKeyAccount::from_random_private_key();

            let address = private_key_account.address.hex_address();
            if validator_clone.validate(&address) {
                found_count_clone.fetch_add(1, RELAXED);
                tx_clone
                    .send((format!("0x{}", address), private_key_account))
                    .unwrap();
            }

            attempt_count_clone.fetch_add(1, RELAXED);
        });

        handles.push(handle);
    }
    drop(tx);
    for (addr, pk) in rx {
        println!(
            "found address {}, private key {}",
            addr,
            pk.secret_key.display_secret()
        );
    }
    status_thread_running.store(false, RELAXED);

    for handle in handles {
        handle.join().unwrap();
    }
    stats_handle.join().unwrap();
    println!("Done");
    println!("Found: {}", found_count.load(RELAXED));
    println!("Attempts: {}", attempt_count.load(RELAXED));
}
