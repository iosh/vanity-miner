use criterion::{criterion_group, criterion_main, Criterion};
use vanity_miner::{
    generator::{AddressFormat, AddressGenerator},
    validator::ValidatorBuilder,
};

fn private_key_with_empty_validator_benchmark_hex(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new().build();
    let address_generator = AddressGenerator::private_key()
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("private_key_with_empty_validator_benchmark_hex", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

fn private_key_with_empty_validator_benchmark_base32(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new().build();
    let address_generator = AddressGenerator::private_key()
        .with_format(AddressFormat::BASE32)
        .with_validator(address_validator)
        .build();

    c.bench_function("private_key_with_empty_validator_benchmark_base32", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

fn private_key_with_empty_validator_benchmark(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new().build();
    let address_generator = AddressGenerator::private_key()
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("private_key_with_empty_validator", |b| {
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

    c.bench_function("private_key_with_prefix_validator", |b| {
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

    c.bench_function("private_key_with_suffix_validator", |b| {
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

    c.bench_function("private_key_with_regex_validator", |b| {
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

    c.bench_function("mnemonic_with_empty_validator_benchmark_hex", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

fn mnemonic_with_empty_validator_benchmark_base32(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new().build();
    let address_generator = AddressGenerator::mnemonic("m/44'/60'/0'/0/0".to_string())
        .with_format(AddressFormat::BASE32)
        .with_validator(address_validator)
        .build();

    c.bench_function("mnemonic_with_empty_validator_benchmark_base32", |b| {
        b.iter(|| address_generator.new_random_address(1029))
    });
}

fn mnemonic_with_empty_validator_benchmark(c: &mut Criterion) {
    let address_validator = ValidatorBuilder::new().build();
    let address_generator = AddressGenerator::mnemonic("m/44'/60'/0'/0/0".to_string())
        .with_format(AddressFormat::HEX)
        .with_validator(address_validator)
        .build();

    c.bench_function("mnemonic_with_empty_validator", |b| {
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

    c.bench_function("mnemonic_with_prefix_validator", |b| {
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

    c.bench_function("mnemonic_with_contains_validator", |b| {
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

    c.bench_function("mnemonic_with_regex_validator", |b| {
        b.iter(|| address_generator.new_random_address(1029))
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

criterion_main!(
    generate_address_by_private_key,
    generate_address_by_mnemonic
);
