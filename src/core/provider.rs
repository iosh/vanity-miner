use std::collections::HashMap;
use std::sync::Arc;

use crate::chains::EthereumChain;

use super::chain::Chain;
use super::types::Result;
use super::types::VanityError;
#[derive(Default)]
pub struct ChainProvider {
    chains: HashMap<String, Arc<dyn Chain>>,
}

impl ChainProvider {
    pub fn new() -> Self {
        let mut provider = Self {
            chains: HashMap::new(),
        };
        provider
            .register(Arc::new(EthereumChain::new()))
            .expect("register Ethereum chain");
        provider
    }

    pub fn register(&mut self, chain: Arc<dyn Chain>) -> Result<()> {
        let id = chain.id().to_owned();
        if self.chains.contains_key(&id) {
            return Err(VanityError::DuplicateChainId(id));
        }
        self.chains.insert(id, chain);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<Arc<dyn Chain>> {
        self.chains.get(id).cloned()
    }

    pub fn list_chains(&self) -> Vec<String> {
        self.chains.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{Address, KeyPair, PublicKey};

    struct DummyChain {
        id: &'static str,
    }

    impl Chain for DummyChain {
        fn id(&self) -> &str {
            self.id
        }

        fn name(&self) -> &str {
            "dummy"
        }

        fn generate_keypair(&self, rng: &mut dyn rand::RngCore) -> Result<KeyPair> {
            Err(crate::core::types::VanityError::CryptoError(
                "unimplemented".into(),
            ))
        }

        fn derive_from_mnemonic(
            &self,
            _mnemonic: &bip39::Mnemonic,
            _path: &bip32::DerivationPath,
            _rng: &mut dyn rand::RngCore,
        ) -> Result<KeyPair> {
            Err(crate::core::types::VanityError::CryptoError(
                "unimplemented".into(),
            ))
        }

        fn keypair_from_secret(&self, secret: &[u8]) -> Result<KeyPair> {
            Err(crate::core::types::VanityError::CryptoError(
                "unimplemented".into(),
            ))
        }

        fn compute_address(&self, public_key: &PublicKey) -> Address {
            Address {
                raw: Vec::new(),
                chain_id: self.id.to_string(),
            }
        }

        fn format_address(
            &self,
            address: &Address,
            config: &crate::core::config::AddressConfig,
        ) -> String {
            String::new()
        }

        fn format_secret(&self, keypair: &KeyPair) -> String {
            String::new()
        }

        fn validate_address(&self, address: &str) -> bool {
            true
        }
    }

    #[test]
    fn register_and_get_chain() {
        let mut provider = ChainProvider::new();
        provider
            .register(Arc::new(DummyChain { id: "eth" }))
            .unwrap();

        assert!(provider.get("eth").is_some());
        assert!(provider.get("cfx").is_none());
    }

    #[test]
    fn reject_duplicate_id() {
        let mut provider = ChainProvider::new();
        provider
            .register(Arc::new(DummyChain { id: "eth" }))
            .unwrap();
        let err = provider
            .register(Arc::new(DummyChain { id: "eth" }))
            .unwrap_err();
        matches!(err, crate::core::types::VanityError::DuplicateChainId(_));
    }
}
