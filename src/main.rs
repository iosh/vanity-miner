mod address;
mod args;
mod validator;
use address::PrivateKeyAccount;

use clap::Parser;
use num_cpus;
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        mpsc, Arc,
    },
    thread,
    time::{Duration, Instant},
};

fn main() {
    let args = args::Args::parse();
    let num_threads = num_cpus::get();
    let mut handles = vec![];

    let max = args.max.unwrap_or(0);
    let limit = args.limit.unwrap_or(0);
    let validator = Arc::new(validator::AddressValidator::new(&args));
    let found_count = Arc::new(AtomicU64::new(0));
    let attempt_count = Arc::new(AtomicU64::new(0));
    let (tx, rx) = mpsc::channel();

    let attempt_count_clone = attempt_count.clone();
    let found_count_clone = found_count.clone();
    let stats_handle = thread::spawn(move || {
        let mut last_attempt = 0u64;
        let mut last_time = Instant::now();

        loop {
            thread::sleep(Duration::from_secs(1));
            let current_time = Instant::now();
            let current_attempt = attempt_count_clone.load(Ordering::Relaxed);
            let current_found = found_count_clone.load(Ordering::Relaxed);

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

            if (max > 0 && current_attempt >= max) || (limit > 0 && current_found >= limit) {
                break;
            }
        }
    });

    for _ in 0..num_threads {
        let tx = tx.clone();
        let validator = validator.clone();
        let found_count = found_count.clone();
        let attempt_count = attempt_count.clone();
        let handle = thread::spawn(move || loop {
            let private_key_account = PrivateKeyAccount::from_random_private_key();

            let address = private_key_account.address.hex_address();
            if validator.validate(&address) {
                found_count.fetch_add(1, Ordering::Relaxed);
                tx.send((format!("0x{}", address), private_key_account))
                    .unwrap();
            }

            attempt_count.fetch_add(1, Ordering::Relaxed);

            if (max > 0 && attempt_count.load(Ordering::Relaxed) >= max)
                || (limit > 0 && found_count.load(Ordering::Relaxed) >= limit)
            {
                break;
            }
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

    for handle in handles {
        handle.join().unwrap();
    }
    stats_handle.join().unwrap();
    println!("Done");
    println!("Found: {}", found_count.load(Ordering::Relaxed));
    println!("Attempts: {}", attempt_count.load(Ordering::Relaxed));
}
