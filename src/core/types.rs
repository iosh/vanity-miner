use std::sync::Arc;

use bip32::DerivationPath;
use bip39::Mnemonic;
use thiserror::Error;
pub type Result<T> = std::result::Result<T, VanityError>;

/// Unified error enum for the vanity miner
#[derive(Error, Debug)]
pub enum VanityError {
    #[error("unsupported chain: {0}")]
    UnsupportedChain(String),

    #[error("invalid derivation path: {0}")]
    InvalidDerivationPath(String),

    #[error("invalid mnemonic: {0}")]
    InvalidMnemonic(String),

    #[error("cryptographic error: {0}")]
    CryptoError(String),

    #[error("duplicate chain id: {0}")]
    DuplicateChainId(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// key pair variants supported by different chains
#[derive(Debug, Clone)]
pub enum KeyPair {
    Secp256k1 {
        secret: [u8; 32],
        public: Vec<u8>,
        mnemonic: Option<Mnemonic>,
        derivation_path: Option<DerivationPath>,
    },
    Ed25519 {
        secret: [u8; 32],
        public: [u8; 32],
        mnemonic: Option<Mnemonic>,
        derivation_path: Option<DerivationPath>,
    },
}

/// Public key container
#[derive(Debug, Clone)]
pub enum PublicKey {
    Secp256k1(Vec<u8>),
    Ed25519([u8; 32]),
}

/// Raw address with chain ID
#[derive(Debug, Clone)]
pub struct Address {
    pub raw: Vec<u8>,
    pub chain_id: String,
}

/// Information emitted when a matching address is found.
#[derive(Debug, Clone)]
pub struct FoundAddress {
    pub address: String,
    pub secret: SecretInfo,
}

/// Secrets can be private keys or mnemonic phrases.
#[derive(Debug, Clone)]
pub enum SecretInfo {
    PrivateKey(String),
    Mnemonic {
        phrase: String,
        derivation_path: String,
    },
}

/// Statistics for a mining run.
#[derive(Debug, Clone)]
pub struct MiningResult {
    pub found: u64,
    pub attempts: u64,
    pub duration_secs: f64,
    pub hashrate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypair_and_secret_info() {
        let kp = KeyPair::Secp256k1 {
            secret: [0u8; 32],
            public: vec![1, 2, 3],
            mnemonic: None,
            derivation_path: None,
        };

        match kp {
            KeyPair::Secp256k1 {
                secret,
                public,
                mnemonic,
                derivation_path,
            } => {
                assert_eq!(secret, [0u8; 32]);
                assert_eq!(public, vec![1, 2, 3]);
                assert!(mnemonic.is_none());
                assert!(derivation_path.is_none());
            }
            _ => panic!("unexpected variant"),
        }

        let secret = SecretInfo::Mnemonic {
            phrase: "test phrase".into(),
            derivation_path: "m/44'/60'/0'/0/0".into(),
        };
        if let SecretInfo::Mnemonic {
            phrase,
            derivation_path,
        } = secret
        {
            assert_eq!(phrase, "test phrase");
            assert_eq!(derivation_path, "m/44'/60'/0'/0/0");
        } else {
            panic!("expected mnemonic variant");
        }
    }
}
