use std::sync::Arc;

use rand::RngCore;
use rayon::{prelude::*, ThreadPoolBuilder};

use crate::{
    core::{
        chain::Chain,
        config::{GenerationMode, MiningConfig},
        executor::Executor,
        types::{Address, FoundAddress, KeyPair, MiningResult, PublicKey, Result, SecretInfo},
    },
    stats::MiningStats,
};

/// Threshold for batching local counters before flushing to shared stats.
const LOCAL_COUNTER_THRESHOLD: u64 = 1000;

/// CPU-base executor
pub struct CpuExecutor {
    threads: usize,
    stats: Arc<MiningStats>,
}

impl CpuExecutor {
    pub fn new(threads: usize, stats: Arc<MiningStats>) -> Self {
        let threads = threads.max(1);
        Self { threads, stats }
    }

    pub fn stats(&self) -> Arc<MiningStats> {
        Arc::clone(&self.stats)
    }

    pub fn threads(&self) -> usize {
        self.threads
    }
}

impl Executor for CpuExecutor {
    fn execute(&self, config: MiningConfig) -> MiningResult {
        let pool = ThreadPoolBuilder::new()
            .num_threads(self.threads)
            .build()
            .expect("failed to build thread pool");

        let stats = Arc::clone(&self.stats);

        // run workers in a dedicated thread pool
        pool.install(|| {
            (0..self.threads)
                .into_par_iter()
                .for_each(|_| worker_loop(config.clone(), Arc::clone(&stats)));
        });

        let snapshot = self.stats.get_snapshot();

        MiningResult {
            found: snapshot.found,
            attempts: snapshot.attempts,
            duration_secs: snapshot.elapsed.as_secs_f64(),
            hashrate: snapshot.hashrate(),
        }
    }
}

fn worker_loop(config: MiningConfig, stats: Arc<MiningStats>) {
    let mut rng = rand::rng();

    let mut local_attempts: u64 = 0;
    let mut local_found: u64 = 0;

    loop {
        let global_attempts = stats.attempts();
        let global_found = stats.found();

        if (config.max_attempts > 0
            && global_attempts.saturating_add(local_attempts) >= config.max_attempts)
            || (config.limit > 0 && global_found.saturating_add(local_found) >= config.limit)
        {
            flush_local_counters(&stats, &mut local_attempts, &mut local_found);
            break;
        }

        let keypair = match generate_keypair_for_mode(
            config.chain.as_ref(),
            &config.generation_mode,
            &mut rng,
        ) {
            Ok(kp) => kp,
            Err(_) => {
                flush_local_counters(&stats, &mut local_attempts, &mut local_found);
                break;
            }
        };

        let public = match public_key_from_keypair(&keypair) {
            Some(pk) => pk,
            None => {
                flush_local_counters(&stats, &mut local_attempts, &mut local_found);
                break;
            }
        };
        let address: Address = config.chain.compute_address(&public);
        let formatted = config
            .chain
            .format_address(&address, &config.address_config);
        let normalized = normalize_address_for_matching(&formatted);

        local_attempts += 1;

        if config.matcher.matches(&normalized) {
            let found = FoundAddress {
                address: formatted,
                secret: build_secret_info(&keypair),
            };

            if config.result_sender.send(found).is_err() {
                flush_local_counters(&stats, &mut local_attempts, &mut local_found);
                break;
            }

            local_found += 1;
        }

        if local_attempts >= LOCAL_COUNTER_THRESHOLD || local_found >= LOCAL_COUNTER_THRESHOLD {
            flush_local_counters(&stats, &mut local_attempts, &mut local_found);
        }
    }
}

fn generate_keypair_for_mode(
    chain: &dyn Chain,
    mode: &GenerationMode,
    rng: &mut dyn RngCore,
) -> Result<KeyPair> {
    match mode {
        GenerationMode::PrivateKey => chain.generate_keypair(rng),
        GenerationMode::Mnemonic {
            word_count,
            derivation_path,
        } => {
            let mnemonic = bip39::Mnemonic::generate(*word_count)
                .map_err(|e| crate::core::types::VanityError::InvalidMnemonic(e.to_string()))?;

            chain.derive_from_mnemonic(&mnemonic, derivation_path, rng)
        }
    }
}

fn public_key_from_keypair(keypair: &KeyPair) -> Option<PublicKey> {
    match keypair {
        KeyPair::Secp256k1 { public, .. } => Some(PublicKey::Secp256k1(public.clone())),
        KeyPair::Ed25519 { public, .. } => Some(PublicKey::Ed25519(*public)),
    }
}

fn build_secret_info(keypair: &KeyPair) -> SecretInfo {
    match keypair {
        KeyPair::Secp256k1 {
            secret,
            mnemonic: Some(m),
            derivation_path: Some(path),
            ..
        }
        | KeyPair::Ed25519 {
            secret,
            mnemonic: Some(m),
            derivation_path: Some(path),
            ..
        } => {
            let _ = secret;
            SecretInfo::Mnemonic {
                phrase: m.to_string(),
                derivation_path: path.to_string(),
            }
        }
        KeyPair::Secp256k1 { secret, .. } | KeyPair::Ed25519 { secret, .. } => {
            SecretInfo::PrivateKey(hex::encode(secret))
        }
    }
}

fn normalize_address_for_matching(address: &str) -> String {
    let stripped = address
        .strip_prefix("0x")
        .or_else(|| address.strip_prefix("0X"))
        .unwrap_or(address);

    let mut normalized = stripped.to_owned();
    normalized.make_ascii_lowercase();
    normalized
}

fn flush_local_counters(stats: &MiningStats, attempts: &mut u64, found: &mut u64) {
    if *attempts > 0 {
        stats.add_attempts(*attempts);
        *attempts = 0;
    }
    if *found > 0 {
        stats.add_found(*found);
        *found = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{mpsc, Arc};

    use crate::core::{
        chain::Chain,
        config::{AddressConfig, ChainConfig, Encoding, GenerationMode, MiningConfig},
        matcher::Matcher,
        types::{Address, KeyPair, PublicKey, Result},
    };

    /// A simple chain implementation for testing the executor.
    #[derive(Debug)]
    struct DummyChain;

    impl Chain for DummyChain {
        fn id(&self) -> &str {
            "dummy"
        }

        fn name(&self) -> &str {
            "Dummy"
        }

        fn generate_keypair(&self, _rng: &mut dyn RngCore) -> Result<KeyPair> {
            Ok(KeyPair::Secp256k1 {
                secret: [0u8; 32],
                public: Vec::new(),
                mnemonic: None,
                derivation_path: None,
            })
        }

        fn derive_from_mnemonic(
            &self,
            _mnemonic: &bip39::Mnemonic,
            _path: &bip32::DerivationPath,
            rng: &mut dyn RngCore,
        ) -> Result<KeyPair> {
            // For tests we just reuse generate_keypair.
            self.generate_keypair(rng)
        }

        fn keypair_from_secret(&self, _secret: &[u8]) -> Result<KeyPair> {
            Ok(KeyPair::Secp256k1 {
                secret: [0u8; 32],
                public: Vec::new(),
                mnemonic: None,
                derivation_path: None,
            })
        }

        fn compute_address(&self, _public_key: &PublicKey) -> Address {
            Address {
                raw: vec![0u8],
                chain_id: "dummy".to_string(),
            }
        }

        fn format_address(&self, _address: &Address, _config: &AddressConfig) -> String {
            "addr".to_string()
        }

        fn format_secret(&self, _keypair: &KeyPair) -> String {
            "secret".to_string()
        }

        fn validate_address(&self, _address: &str) -> bool {
            true
        }
    }

    /// Matcher that always returns the configured boolean.
    #[derive(Debug)]
    struct AlwaysMatch(bool);

    impl Matcher for AlwaysMatch {
        fn matches(&self, _address: &str) -> bool {
            self.0
        }

        fn description(&self) -> String {
            format!("always({})", self.0)
        }
    }

    #[test]
    fn cpu_executor_respects_max_attempts_single_thread() {
        let stats = Arc::new(MiningStats::new());
        let executor = CpuExecutor::new(1, stats.clone());

        let (tx, rx) = mpsc::channel();

        let chain = Arc::new(DummyChain) as Arc<dyn Chain>;
        let matcher = Arc::new(AlwaysMatch(false)) as Arc<dyn Matcher>;

        let address_config = AddressConfig {
            encoding: Encoding::Hex,
            chain_config: ChainConfig::Ethereum { checksum: false },
        };

        let config = MiningConfig {
            chain,
            matcher,
            generation_mode: GenerationMode::PrivateKey,
            address_config,
            max_attempts: 100,
            limit: 0,
            result_sender: tx,
        };

        let result = executor.execute(config);

        assert_eq!(result.found, 0);
        assert_eq!(result.attempts, 100);

        // No results should have been sent.
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn cpu_executor_respects_limit_single_thread() {
        let stats = Arc::new(MiningStats::new());
        let executor = CpuExecutor::new(1, stats.clone());

        let (tx, rx) = mpsc::channel();

        let chain = Arc::new(DummyChain) as Arc<dyn Chain>;
        let matcher = Arc::new(AlwaysMatch(true)) as Arc<dyn Matcher>;

        let address_config = AddressConfig {
            encoding: Encoding::Hex,
            chain_config: ChainConfig::Ethereum { checksum: false },
        };

        let config = MiningConfig {
            chain,
            matcher,
            generation_mode: GenerationMode::PrivateKey,
            address_config,
            max_attempts: 10_000,
            limit: 5,
            result_sender: tx,
        };

        let result = executor.execute(config);

        assert_eq!(result.found, 5);
        assert!(result.attempts >= 5);

        // Exactly 5 results should have been sent.
        let mut received = 0;
        while rx.try_recv().is_ok() {
            received += 1;
        }
        assert_eq!(received, 5);
    }
}
