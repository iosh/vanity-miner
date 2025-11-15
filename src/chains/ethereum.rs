use bip32::{DerivationPath, XPrv};
use bip39::Mnemonic;
use rand::RngCore;
use secp256k1::{PublicKey as SecpPubkey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

use crate::core::{
    chain::Chain,
    config::{AddressConfig, ChainConfig, Encoding},
    types::{Address, KeyPair, PublicKey, Result, VanityError},
};

pub struct EthereumChain {
    secp: Secp256k1<secp256k1::All>,
}

impl EthereumChain {
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
        }
    }

    fn public_key_bytes(public_key: &PublicKey) -> Result<Vec<u8>> {
        match public_key {
            PublicKey::Secp256k1(bytes) => Ok(bytes.clone()),
            PublicKey::Ed25519(_) => Err(VanityError::CryptoError(
                "Ethereum only supports secp256k1 keys".into(),
            )),
        }
    }

    fn keccak256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    fn derive_pubkey_bytes(&self, secret_key: &SecretKey) -> Vec<u8> {
        SecpPubkey::from_secret_key(&self.secp, secret_key)
            .serialize_uncompressed()
            .to_vec()
    }

    fn hex_lower(address: &Address) -> String {
        hex::encode(&address.raw)
    }

    fn eip55_checksum(address: &Address) -> String {
        let lower = Self::hex_lower(address);
        let hash = Self::keccak256(lower.as_bytes());

        let mut checksum = String::with_capacity(lower.len());

        for (i, c) in lower.chars().enumerate() {
            if c.is_ascii_digit() {
                checksum.push(c);
                continue;
            }

            let byte = hash[i / 2];
            let nibble = if i % 2 == 0 { byte >> 4 } else { byte & 0x0F };
            if nibble >= 8 {
                checksum.push(c.to_ascii_uppercase());
            } else {
                checksum.push(c);
            }
        }
        checksum
    }

    fn strip_0x(address: &str) -> &str {
        address
            .strip_prefix("0x")
            .or_else(|| address.strip_prefix("0X"))
            .unwrap_or(address)
    }
}

impl Chain for EthereumChain {
    fn id(&self) -> &str {
        "ethereum"
    }

    fn name(&self) -> &str {
        "Ethereum"
    }

    fn compute_address(&self, public_key: &crate::core::types::PublicKey) -> Address {
        let bytes = Self::public_key_bytes(public_key).expect("secp256k1 only");
        debug_assert_eq!(bytes.len(), 65);
        debug_assert_eq!(bytes[0], 0x04);

        let hash = Self::keccak256(&bytes[1..]);
        let raw = hash[12..].to_vec();

        Address {
            raw,
            chain_id: "ethereum".into(),
        }
    }

    fn format_address(&self, address: &Address, config: &AddressConfig) -> String {
        match (&config.encoding, &config.chain_config) {
            (Encoding::HexChecksum, ChainConfig::Ethereum { checksum: true }) => {
                format!("0x{}", Self::eip55_checksum(address))
            }
            _ => format!("0x{}", Self::hex_lower(address)),
        }
    }

    fn validate_address(&self, address: &str) -> bool {
        let normalized = Self::strip_0x(address);
        if normalized.len() != 40 || !normalized.chars().all(|c| c.is_ascii_hexdigit()) {
            return false;
        }

        let is_all_lower = normalized
            .chars()
            .all(|c| !c.is_ascii_alphabetic() || c.is_ascii_lowercase());

        let is_all_upper = normalized
            .chars()
            .all(|c| !c.is_ascii_alphabetic() || c.is_ascii_uppercase());

        // if the address is all lowercase or all uppercase, it is valid (not checksummed)
        if is_all_lower || is_all_upper {
            return true;
        }

        let raw = match hex::decode(normalized.to_ascii_lowercase()) {
            Ok(r) => r,
            Err(_) => return false,
        };

        if raw.len() != 20 {
            return false;
        }

        let addr = Address {
            raw,
            chain_id: "ethereum".into(),
        };

        Self::eip55_checksum(&addr) == normalized
    }

    fn derive_from_mnemonic(&self, mnemonic: &Mnemonic, path: &DerivationPath) -> Result<KeyPair> {
        let seed = mnemonic.to_seed("");

        let xprv = XPrv::derive_from_path(seed, path)
            .map_err(|e| VanityError::CryptoError(e.to_string()))?;

        let child_key = xprv.private_key();
        let secret_bytes = child_key.to_bytes();

        let secret = SecretKey::from_slice(&secret_bytes)
            .map_err(|e| VanityError::CryptoError(e.to_string()))?;

        let public = self.derive_pubkey_bytes(&secret);

        Ok(KeyPair::Secp256k1 {
            secret: secret.secret_bytes(),
            public,
            mnemonic: Some(mnemonic.clone()),
            derivation_path: Some(path.clone()),
        })
    }

    fn format_secret(&self, keypair: &KeyPair) -> String {
        match keypair {
            KeyPair::Secp256k1 {
                secret,
                mnemonic,
                derivation_path,
                ..
            } => {
                if let (Some(m), Some(path)) = (mnemonic, derivation_path) {
                    return format!("mnemonic:{}|path:{}", m.to_string(), path.to_string());
                } else {
                    hex::encode(secret)
                }
            }
            KeyPair::Ed25519 { .. } => String::from("<unsupported>"),
        }
    }
    fn generate_keypair(&self, rng: &mut dyn RngCore) -> Result<KeyPair> {
        let mut secret_bytes = [0u8; 32];

        let secret = loop {
            rng.fill_bytes(&mut secret_bytes);
            if let Ok(sec) = SecretKey::from_byte_array(&secret_bytes) {
                break sec;
            }
        };

        let pubkey = self.derive_pubkey_bytes(&secret);

        Ok(KeyPair::Secp256k1 {
            secret: secret.secret_bytes(),
            public: pubkey,
            derivation_path: None,
            mnemonic: None,
        })
    }

    fn keypair_from_secret(&self, secret: &[u8]) -> Result<KeyPair> {
        if secret.len() != 32 {
            return Err(VanityError::CryptoError("invalid secret length".into()));
        }

        let secret_key = SecretKey::from_slice(secret)
            .map_err(|e| VanityError::CryptoError(format!("invalid secret: {}", e)))?;

        let public = self.derive_pubkey_bytes(&secret_key);

        Ok(KeyPair::Secp256k1 {
            secret: secret_key.secret_bytes(),
            public,
            mnemonic: None,
            derivation_path: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bip32::DerivationPath;
    use hex::decode;
    use rand::{rngs::StdRng, SeedableRng};

    fn test_chain() -> EthereumChain {
        EthereumChain::new()
    }

    #[test]
    fn compute_address_matches_expected() {
        let chain = test_chain();
        let secret_bytes =
            decode("c3e92bc305f6197098645e9fabee4cdad96235b0ccb8fb1e9089b620433af6cf")
                .expect("hex");

        let secret = SecretKey::from_slice(&secret_bytes).expect("valid secret");

        let public = chain.derive_pubkey_bytes(&secret);

        let key = PublicKey::Secp256k1(public);

        let address = chain.compute_address(&key);

        assert_eq!(
            hex::encode(&address.raw),
            "61b5ccbbee987149006b93a9cf5be30706b610e0"
        )
    }

    #[test]
    fn format_address_handle_checksum() {
        let chain = test_chain();

        let raw = decode("61b5ccbbee987149006b93a9cf5be30706b610e0").expect("hex");

        let address = Address {
            raw,
            chain_id: "ethereum".into(),
        };

        let plain_cfg = AddressConfig {
            encoding: Encoding::Hex,
            chain_config: ChainConfig::Ethereum { checksum: false },
        };

        assert_eq!(
            chain.format_address(&address, &plain_cfg),
            "0x61b5ccbbee987149006b93a9cf5be30706b610e0"
        );

        let checksum_cfg = AddressConfig {
            encoding: Encoding::HexChecksum,
            chain_config: ChainConfig::Ethereum { checksum: true },
        };

        assert_eq!(
            chain.format_address(&address, &checksum_cfg),
            "0x61B5ccbBee987149006B93a9Cf5bE30706B610e0"
        )
    }

    #[test]
    fn format_secret_outputs_hex_or_mnemonic() {
        let chain = test_chain();

        let key_private = KeyPair::Secp256k1 {
            secret: [0u8; 32],
            public: vec![],
            mnemonic: None,
            derivation_path: None,
        };
        assert_eq!(
            chain.format_secret(&key_private),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );

        let mnemonic = Mnemonic::parse(
            "abandon abandon abandon abandon abandon abandon abandon abandon
  abandon abandon abandon about",
        )
        .expect("valid mnemonic");
        let path: DerivationPath = "m/44'/60'/0'/0/0".parse().expect("valid path");

        let key_mnemonic = KeyPair::Secp256k1 {
            secret: [0u8; 32],
            public: vec![],
            mnemonic: Some(mnemonic.clone()),
            derivation_path: Some(path.clone()),
        };

        let formatted = chain.format_secret(&key_mnemonic);
        assert!(formatted.contains(mnemonic.to_string().as_str()));
        assert!(formatted.contains(&path.to_string()));
    }

    #[test]
    fn generate_keypair_is_deterministic_with_seed() {
        let chain = test_chain();
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        let kp1 = chain.generate_keypair(&mut rng1).expect("keypair");
        let kp2 = chain.generate_keypair(&mut rng2).expect("keypair");

        match (kp1, kp2) {
            (
                KeyPair::Secp256k1 {
                    secret: s1,
                    public: p1,
                    ..
                },
                KeyPair::Secp256k1 {
                    secret: s2,
                    public: p2,
                    ..
                },
            ) => {
                assert_eq!(s1, s2);
                assert_eq!(p1, p2);
                assert_eq!(p1.len(), 65);
                assert_eq!(p1[0], 0x04);
            }
            _ => panic!("unexpected key type"),
        }
    }

    #[test]
    fn derive_from_mnemonic_matches_vector() {
        let chain = test_chain();
        let mnemonic_str = "test test test test test test test test test test test junk";
        let mnemonic = Mnemonic::parse(mnemonic_str).expect("valid mnemonic");

        let path: DerivationPath = "m/44'/60'/0'/0/0".parse().expect("valid derivation path");

        let keypair = chain
            .derive_from_mnemonic(&mnemonic, &path)
            .expect("mnemonic derivation");

        if let KeyPair::Secp256k1 {
            secret,
            public,
            mnemonic: m,
            derivation_path,
            ..
        } = keypair
        {
            assert_eq!(
                hex::encode(secret),
                "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
            );
            assert!(public.len() == 65 && public[0] == 0x04);
            assert_eq!(m.unwrap().to_string(), mnemonic_str);
            assert_eq!(derivation_path.unwrap().to_string(), path.to_string());

            let address = chain.compute_address(&PublicKey::Secp256k1(public));
            assert_eq!(
                hex::encode(address.raw),
                "f39fd6e51aad88f6f4ce6ab8827279cfffb92266"
            );
        } else {
            panic!("unexpected key type");
        }
    }

    #[test]
    fn validate_address_covers_edge_cases() {
        let chain = test_chain();
        let lowercase = "0x61b5ccbbee987149006b93a9cf5be30706b610e0";
        let checksum = "0x61B5ccbBee987149006B93a9Cf5bE30706B610e0";
        let wrong_checksum = "0x61B5ccbBee987149006B93a9Cf5bE30706B610e1";
        let bad_chars = "0x61b5ccbbee987149006b93a9cf5be30706b610ez";
        let short = "0x1234";

        assert!(chain.validate_address(lowercase));
        assert!(chain.validate_address(checksum));
        assert!(!chain.validate_address(wrong_checksum));
        assert!(!chain.validate_address(bad_chars));
        assert!(!chain.validate_address(short));
        assert!(chain.validate_address("61B5CCBBEE987149006B93A9CF5BE30706B610E0"));
    }

    #[test]
    fn keypair_from_secret_yields_expected_public() {
        let chain = test_chain();
        let secret_bytes =
            decode("c3e92bc305f6197098645e9fabee4cdad96235b0ccb8fb1e9089b620433af6cf")
                .expect("hex");
        let keypair = chain
            .keypair_from_secret(&secret_bytes)
            .expect("keypair from secret");

        if let KeyPair::Secp256k1 {
            secret,
            public,
            mnemonic,
            derivation_path,
        } = keypair
        {
            assert_eq!(
                hex::encode(secret),
                "c3e92bc305f6197098645e9fabee4cdad96235b0ccb8fb1e9089b620433af6cf"
            );
            assert_eq!(public.len(), 65);
            assert_eq!(public[0], 0x04);
            assert!(mnemonic.is_none());
            assert!(derivation_path.is_none());
        } else {
            panic!("unexpected key type");
        }
    }
}
