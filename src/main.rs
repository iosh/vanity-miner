mod address;

use address::PrivateKeyAccount;
use clap::Parser;
use num_cpus;
use std::thread;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Maximum number of times to generate addresses.
    #[arg(short, long)]
    max: Option<u64>,

    /// Maximum number of results to return.
    #[arg(short, long)]
    limit: Option<u64>,

    /// Maximum number of concurrent threads to use.
    #[arg(short, long)]
    threads: Option<u16>,

    /// Strings that the address must contain (supports multiple strings, separated by commas).
    #[arg(short, long)]
    contains: Vec<String>,

    /// Prefix that the address must match.
    #[arg(short, long)]
    prefix: Option<String>,

    /// Suffix that the address must match.
    #[arg(short, long)]
    suffix: Option<String>,

    /// Regular expression that the address must match.
    #[arg(short, long)]
    regex: Option<String>,
}

fn main() {
    let args = Args::parse();
    let num_threads = num_cpus::get();
    let mut handles = vec![];

    for _ in 0..num_threads {
        let handle = thread::spawn(move || {
            let private_key_account = PrivateKeyAccount::from_random_private_key();

            let _ = private_key_account.address.display_hex_address();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
