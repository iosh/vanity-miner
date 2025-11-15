use criterion::{criterion_group, criterion_main, Criterion};
use vanity_miner::{
    generator::{AddressFormat, AddressGenerator},
    validator::ValidatorBuilder,
};

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

fn private_key_with_empty_validator_benchmark_hex(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new().build();
    let address_generator = AddressGenerator::private_key()
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("generator_private_key_empty_validator_hex_xN", |b| {
        b.iter(|| {
            for _ in 0..PRIVATE_ATTEMPTS_PER_ITER {
                criterion::black_box(address_generator.new_random_address(1029));
            }
        })
    });
}

fn private_key_with_empty_validator_benchmark_base32(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new().build();
    let address_generator = AddressGenerator::private_key()
        .with_format(AddressFormat::BASE32)
        .with_validator(address_validator)
        .build();

    c.bench_function("generator_private_key_empty_validator_base32", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

fn private_key_with_empty_validator_benchmark(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new().build();
    let address_generator = AddressGenerator::private_key()
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("generator_private_key_empty_validator", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

fn private_key_with_prefix_validator_benchmark(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new()
        .with_prefix("123".to_string())
        .build();
    let address_generator = AddressGenerator::private_key()
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("generator_private_key_prefix_validator", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

fn private_key_with_contains_validator_benchmark(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new()
        .with_contains(vec![
            "9ace".to_string(),
            "aaaa".to_string(),
            "999999999".to_string(),
            "ccccc".to_string(),
        ])
        .build();
    let address_generator = AddressGenerator::private_key()
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("generator_private_key_contains_validator", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

fn private_key_with_regex_validator_benchmark(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new()
        .with_regex("0{10}".to_string())
        .build();
    let address_generator = AddressGenerator::private_key()
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("generator_private_key_regex_validator", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

criterion_group!(
    generate_address_by_private_key,
    private_key_with_empty_validator_benchmark_hex,
    private_key_with_empty_validator_benchmark_base32,
    private_key_with_empty_validator_benchmark,
    private_key_with_prefix_validator_benchmark,
    private_key_with_contains_validator_benchmark,
    private_key_with_regex_validator_benchmark
);

fn mnemonic_with_empty_validator_benchmark_hex(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new().build();
    let address_generator = AddressGenerator::mnemonic("m/44'/60'/0'/0/0".to_string())
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("generator_mnemonic_empty_validator_hex_xN", |b| {
        b.iter(|| {
            for _ in 0..MNEMONIC_ATTEMPTS_PER_ITER {
                criterion::black_box(address_generator.new_random_address(1029));
            }
        })
    });
}

fn mnemonic_with_empty_validator_benchmark_base32(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new().build();
    let address_generator = AddressGenerator::mnemonic("m/44'/60'/0'/0/0".to_string())
        .with_format(AddressFormat::BASE32)
        .with_validator(address_validator)
        .build();

    c.bench_function("generator_mnemonic_empty_validator_base32", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

fn mnemonic_with_empty_validator_benchmark(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new().build();
    let address_generator = AddressGenerator::mnemonic("m/44'/60'/0'/0/0".to_string())
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("generator_mnemonic_empty_validator", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

fn mnemonic_with_prefix_validator_benchmark(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new()
        .with_prefix("123".to_string())
        .build();
    let address_generator = AddressGenerator::mnemonic("m/44'/60'/0'/0/0".to_string())
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("generator_mnemonic_prefix_validator", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

fn mnemonic_with_contains_validator_benchmark(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new()
        .with_contains(vec![
            "9ace".to_string(),
            "aaaa".to_string(),
            "999999999".to_string(),
            "ccccc".to_string(),
        ])
        .build();
    let address_generator = AddressGenerator::mnemonic("m/44'/60'/0'/0/0".to_string())
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("generator_mnemonic_contains_validator", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

fn mnemonic_with_regex_validator_benchmark(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new()
        .with_regex("0{10}".to_string())
        .build();
    let address_generator = AddressGenerator::mnemonic("m/44'/60'/0'/0/0".to_string())
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("generator_mnemonic_regex_validator", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

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
    name = generate_address_by_mnemonic;
    config = Criterion::default().sample_size(50);
    targets = mnemonic_with_empty_validator_benchmark_hex,
    mnemonic_with_empty_validator_benchmark_base32,
    mnemonic_with_empty_validator_benchmark,
    mnemonic_with_prefix_validator_benchmark,
    mnemonic_with_contains_validator_benchmark,
    mnemonic_with_regex_validator_benchmark
);

criterion_group!(
    cpu_executor_group,
    cpu_executor_private_key_benchmark,
    cpu_executor_mnemonic_benchmark
);

criterion_main!(
    generate_address_by_private_key,
    generate_address_by_mnemonic,
    cpu_executor_group
);
