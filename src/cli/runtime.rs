use std::sync::Arc;

use super::Args as CliArgs;
use crate::{
    core::{
        chain::Chain,
        config::{AddressConfig, ChainConfig, Encoding, GenerationMode},
        matcher::DynMatcher,
        types::{Result, VanityError},
        ChainProvider,
    },
    matchers::{build_matcher, MatcherConfig},
};
use bip32::DerivationPath;
use num_cpus;

/// Fully resolved runtime configuration derived from CLI args.
pub struct RuntimeConfig {
    pub chain: Arc<dyn Chain>,
    pub matcher: DynMatcher,
    pub generation_mode: GenerationMode,
    pub address_config: AddressConfig,
    pub max_attempts: u64,
    pub limit: u64,
    pub threads: usize,
    pub output: String,
    pub console: bool,
    pub no_file: bool,
}

/// Build a `RuntimeConfig` from parsed CLI arguments.
pub fn build_runtime_config(args: &CliArgs) -> Result<RuntimeConfig> {
    let provider = ChainProvider::new();
    let chain = provider
        .get(&args.chain)
        .ok_or_else(|| VanityError::UnsupportedChain(args.chain.clone()))?;

    let mut matcher_cfg = MatcherConfig::default();

    if let Some(prefix) = args.prefix.clone() {
        matcher_cfg.prefix = Some(prefix);
    }

    if let Some(suffix) = args.suffix.clone() {
        matcher_cfg.suffix = Some(suffix);
    }

    if let Some(contains) = args.contains.clone() {
        matcher_cfg.contains = contains;
    }

    if let Some(regex) = args.regex.clone() {
        matcher_cfg.regex = Some(regex);
    }

    let matcher = build_matcher(matcher_cfg)
        .map_err(|e| VanityError::CryptoError(format!("invalid regex: {e}")))?;

    let generation_mode = if args.private_key {
        GenerationMode::PrivateKey
    } else {
        let word_count = args.mnemonic_words.unwrap_or(12);
        let path: DerivationPath = args
            .derivation_path
            .parse()
            .map_err(|e: bip32::Error| VanityError::InvalidDerivationPath(e.to_string()))?;

        GenerationMode::Mnemonic {
            word_count,
            derivation_path: path,
        }
    };

    let address_config = AddressConfig::new(
        Encoding::HexChecksum,
        ChainConfig::Ethereum { checksum: true },
    );

    // 5. Limits and threads.
    let max_attempts = args.max_attempts.unwrap_or(0);
    let limit = args.limit.unwrap_or(0);
    let threads = args.threads.unwrap_or_else(num_cpus::get);

    Ok(RuntimeConfig {
        chain,
        matcher,
        generation_mode,
        address_config,
        max_attempts,
        limit,
        threads,
        output: args.output.clone(),
        console: args.console,
        no_file: args.no_file,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Args;

    #[test]
    fn runtime_config_basic_fields_are_resolved() {
        let args = Args {
            chain: "ethereum".to_string(),
            private_key: true,
            mnemonic: false,
            max_attempts: Some(100),
            limit: Some(5),
            threads: Some(2),
            contains: None,
            prefix: Some("dead".into()),
            suffix: None,
            regex: None,
            derivation_path: "m/44'/60'/0'/0/0".into(),
            mnemonic_words: None,
            output: "vanity-addresses.csv".into(),
            console: false,
            no_file: false,
        };

        let runtime = build_runtime_config(&args).expect("runtime config");
        assert_eq!(runtime.max_attempts, 100);
        assert_eq!(runtime.limit, 5);
        assert_eq!(runtime.threads, 2);
    }
}
