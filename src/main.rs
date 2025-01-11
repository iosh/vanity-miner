mod address;
mod cli;
mod progress_monitor;
mod validator;
use address::{MnemonicAccount, PrivateKeyAccount};
use bip32::DerivationPath;
use clap::Parser;
use num_cpus;
use progress_monitor::GenerationStats;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::{Duration, Instant},
};

static RELAXED: Ordering = Ordering::Relaxed;

fn main() {
    let args = cli::Args::parse();
    let num_threads = args.threads.unwrap_or(num_cpus::get());
    let mut handles = vec![];

    let stats = GenerationStats::new(args.max_attempts.unwrap_or(0), args.limit.unwrap_or(0));

    let validator = Arc::new(validator::AddressValidator::new(&args));
    let (tx, rx) = mpsc::channel();

    let attempt_count_clone = stats.attempt_count.clone();
    let found_count_clone = stats.found_count.clone();

    let status_thread_running = Arc::new(AtomicBool::new(true));

    let status_thread_running_clone = status_thread_running.clone();

    let path: Arc<DerivationPath> = Arc::new(
        args.derivation_path
            .parse()
            .map_err(|e| format!("Invalid derivation path: {}", e))
            .unwrap(),
    );

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
        let found_count_clone = stats.found_count.clone();
        let attempt_count_clone = stats.attempt_count.clone();
        let path_clone = path.clone();

        let handle = thread::spawn(move || loop {
            if (stats.max_attempts > 0 && attempt_count_clone.load(RELAXED) >= stats.max_attempts)
                || (stats.max_matches > 0 && found_count_clone.load(RELAXED) >= stats.max_matches)
            {
                break;
            }

            let (address, secret) = if args.from_private_key {
                let private_key_account = PrivateKeyAccount::from_random_private_key();
                (
                    private_key_account.address.hex_address(),
                    private_key_account.secret_key.display_secret().to_string(),
                )
            } else {
                let mnemonic_account = MnemonicAccount::from_random_mnemonic(12, &path_clone);
                (
                    mnemonic_account.address.hex_address(),
                    mnemonic_account.mnemonic.to_string(),
                )
            };

            if validator_clone.validate(&address) {
                found_count_clone.fetch_add(1, RELAXED);
                tx_clone
                    .send((format!("0x{}", address), secret.to_string()))
                    .unwrap();
            }

            attempt_count_clone.fetch_add(1, RELAXED);
        });

        handles.push(handle);
    }
    drop(tx);
    for (addr, pk) in rx {
        println!("found address {}, private key {}", addr, pk);
    }
    status_thread_running.store(false, RELAXED);

    for handle in handles {
        handle.join().unwrap();
    }
    stats_handle.join().unwrap();
    println!("Done");
    println!("Found: {}", stats.found_count.load(RELAXED));
    println!("Attempts: {}", stats.attempt_count.load(RELAXED));
}
