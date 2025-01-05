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

    for _ in 0..num_threads {
        let tx = tx.clone();
        let validator = validator.clone();
        let found_count = found_count.clone();
        let attempt_count = attempt_count.clone();
        let handle = thread::spawn(move || {
            let mut need_loop = true;

            while need_loop {
                let private_key_account = PrivateKeyAccount::from_random_private_key();

                let address = private_key_account.address.hex_address();
                if validator.validate(&address) {
                    found_count.fetch_add(1, Ordering::Relaxed);
                    tx.send((format!("0x{}", address), private_key_account))
                        .unwrap();
                }

                attempt_count.fetch_add(1, Ordering::Relaxed);

                if max > 0 && attempt_count.load(Ordering::Relaxed) >= max {
                    need_loop = false;
                }

                if limit > 0 && found_count.load(Ordering::Relaxed) >= limit {
                    need_loop = false;
                }
            }
        });

        handles.push(handle);
    }
    drop(tx);
    for (addr, pk) in rx {
        println!("found address {}, private key {}", addr, pk.secret_key.display_secret());
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Done");
    println!("Found: {}", found_count.load(Ordering::Relaxed));
    println!("Attempts: {}", attempt_count.load(Ordering::Relaxed));
}
