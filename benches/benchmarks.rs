use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::{mpsc, Arc};

use bip32::DerivationPath;
use vanity_miner::core::executor::Executor;
use vanity_miner::{
    chains::EthereumChain,
    core::{
        chain::Chain,
        config::{AddressConfig, ChainConfig, Encoding, GenerationMode, MiningConfig},
        matcher::Matcher,
    },
    executors::CpuExecutor,
    matchers::RegexMatcher,
    stats::MiningStats,
};

const PRIVATE_ATTEMPTS_PER_ITER: u64 = 500;
const MNEMONIC_ATTEMPTS_PER_ITER: u64 = 50;

fn cpu_executor_private_key_benchmark(c: &mut Criterion) {
    c.bench_function("cpu_executor_private_key_eth_1_thread_xN", |b| {
        b.iter(|| {
            let stats = Arc::new(MiningStats::new());
            let executor = CpuExecutor::new(1, stats.clone());

            let (tx, _rx) = mpsc::channel();

            let chain: Arc<dyn Chain> = Arc::new(EthereumChain::new());
            let matcher: Arc<dyn Matcher> = Arc::new(RegexMatcher::new("^$").expect("valid regex"));

            let address_config = AddressConfig {
                encoding: Encoding::Hex,
                chain_config: ChainConfig::Ethereum { checksum: false },
            };

            let config = MiningConfig {
                chain,
                matcher,
                generation_mode: GenerationMode::PrivateKey,
                address_config,
                max_attempts: PRIVATE_ATTEMPTS_PER_ITER,
                limit: 0,
                result_sender: tx,
            };

            let result = executor.execute(config);
            criterion::black_box(result);
        })
    });
}
fn cpu_executor_mnemonic_benchmark(c: &mut Criterion) {
    c.bench_function("cpu_executor_mnemonic_eth_1_thread_xN", |b| {
        b.iter(|| {
            let stats = Arc::new(MiningStats::new());
            let executor = CpuExecutor::new(1, stats.clone());

            let (tx, _rx) = mpsc::channel();

            let chain: Arc<dyn Chain> = Arc::new(EthereumChain::new());
            let matcher: Arc<dyn Matcher> = Arc::new(RegexMatcher::new("^$").expect("valid regex"));

            let address_config = AddressConfig {
                encoding: Encoding::Hex,
                chain_config: ChainConfig::Ethereum { checksum: false },
            };

            let config = MiningConfig {
                chain,
                matcher,
                generation_mode: GenerationMode::Mnemonic {
                    word_count: 12,
                    derivation_path: "m/44'/60'/0'/0/0"
                        .parse::<DerivationPath>()
                        .expect("valid derivation path"),
                },
                address_config,
                max_attempts: MNEMONIC_ATTEMPTS_PER_ITER,
                limit: 0,
                result_sender: tx,
            };

            let result = executor.execute(config);
            criterion::black_box(result);
        })
    });
}

criterion_group!(
    cpu_executor_group,
    cpu_executor_private_key_benchmark,
    cpu_executor_mnemonic_benchmark
);

criterion_main!(cpu_executor_group);
