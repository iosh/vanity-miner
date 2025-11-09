use std::sync::{mpsc, Arc};

use super::chain::Chain;
use super::matcher::Matcher;
use super::types::FoundAddress;

/// Supported output encodings for formatted addresses.
#[derive(Clone, Debug)]
pub enum Encoding {
    Hex,
    HexChecksum,
    Base32,
    Base58,
    Base64,
    Custom(String),
}

/// Chain-specific formatting options
#[derive(Debug, Clone)]
pub enum ChainConfig {
    Ethereum { checksum: bool },
    Conflux { network_id: u32 },
}

/// Address formatting configuration passed down to chains.
#[derive(Debug, Clone)]
pub struct AddressConfig {
    pub encoding: Encoding,
    pub chain_config: ChainConfig,
}

impl AddressConfig {
    pub fn new(encoding: Encoding, chain_config: ChainConfig) -> Self {
        Self {
            encoding,
            chain_config,
        }
    }
}

/// Controls how key material is generated.
#[derive(Debug, Clone)]
pub enum GenerationMode {
    PrivateKey,
    Mnemonic {
        word_count: usize,
        derivation_path: String,
    },
}

/// Full configuration consumed by executors.
#[derive(Clone)]
pub struct MiningConfig {
    pub chain: Arc<dyn Chain>,
    pub matcher: Arc<dyn Matcher>,
    pub generation_mode: GenerationMode,
    pub address_config: AddressConfig,
    pub max_attempts: u64,
    pub limit: u64,
    pub result_sender: mpsc::Sender<FoundAddress>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::matcher::Matcher;
    use crate::core::types::{Address, KeyPair, PublicKey, Result, VanityError};

    struct TestChain;

    impl crate::core::chain::Chain for TestChain {
        fn id(&self) -> &str {
            "test"
        }
        fn name(&self) -> &str {
            "TestChain"
        }
        fn generate_keypair(&self, _rng: &mut dyn rand::RngCore) -> Result<KeyPair> {
            Err(VanityError::CryptoError("unimplemented".into()))
        }
        fn derive_from_mnemonic(
            &self,
            _mnemonic: &str,
            _path: &str,
            _rng: &mut dyn rand::RngCore,
        ) -> Result<KeyPair> {
            Err(VanityError::CryptoError("unimplemented".into()))
        }
        fn keypair_from_secret(&self, _secret: &[u8]) -> Result<KeyPair> {
            Err(VanityError::CryptoError("unimplemented".into()))
        }
        fn compute_address(&self, _public_key: &PublicKey) -> Address {
            Address {
                raw: Vec::new(),
                chain_id: "test".into(),
            }
        }
        fn format_address(&self, _address: &Address, _config: &AddressConfig) -> String {
            "addr".into()
        }
        fn format_secret(&self, _keypair: &KeyPair) -> String {
            "secret".into()
        }
        fn validate_address(&self, _address: &str) -> bool {
            true
        }
    }

    struct TestMatcher;
    impl Matcher for TestMatcher {
        fn matches(&self, _address: &str) -> bool {
            true
        }
        fn description(&self) -> String {
            "always".into()
        }
    }

    #[test]
    fn address_config_new() {
        let cfg = AddressConfig::new(Encoding::Hex, ChainConfig::Ethereum { checksum: true });
        matches!(cfg.chain_config, ChainConfig::Ethereum { checksum: true });
    }

    #[test]
    fn mining_config_fields() {
        let (tx, _rx) = mpsc::channel();
        let config = MiningConfig {
            chain: Arc::new(TestChain),
            matcher: Arc::new(TestMatcher),
            generation_mode: GenerationMode::PrivateKey,
            address_config: AddressConfig::new(
                Encoding::Hex,
                ChainConfig::Ethereum { checksum: false },
            ),
            max_attempts: 100,
            limit: 5,
            result_sender: tx,
        };

        assert_eq!(config.max_attempts, 100);
        assert_eq!(config.limit, 5);
    }
}
