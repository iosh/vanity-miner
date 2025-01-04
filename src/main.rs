mod address;

use std::thread;
use address::PrivateKeyAccount;
use num_cpus;

fn main() {
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
