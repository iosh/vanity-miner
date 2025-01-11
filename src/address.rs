use std::fmt::Display;

use bip32::{DerivationPath, XPrv};
use bip39::{Language, Mnemonic};
use secp256k1::rand::rngs::ThreadRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

pub type EthereumAddressBytes = [u8; 20];

pub struct Address {
    address: EthereumAddressBytes,
}

pub struct PrivateKeyAccount {
    pub secret_key: SecretKey,
    pub address: Address,
}

pub struct MnemonicAccount {
    pub mnemonic: String,
    pub address: Address,
}

impl MnemonicAccount {
    pub fn from_random_mnemonic(word_count: usize, path: &DerivationPath) -> Self {
        let mnemonic = Mnemonic::generate(word_count).unwrap();
        let seed = mnemonic.to_seed("");

        let private_key = XPrv::derive_from_path(&seed, path)
            .map_err(|e| format!("Failed to derive private key: {}", e))
            .unwrap();

        let public_key = private_key
            .public_key()
            .public_key()
            .to_encoded_point(false);
        MnemonicAccount {
            mnemonic: mnemonic.to_string(),
            address: Address::from_public_key(&public_key.as_bytes()),
        }
    }
}

impl PrivateKeyAccount {
    pub fn from_random_private_key() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut ThreadRng::default());

        PrivateKeyAccount {
            secret_key,
            address: Address::from_public_key(&public_key.serialize_uncompressed()),
        }
    }
}

impl Address {
    pub fn from_public_key(public_key: &[u8]) -> Self {
        let mut hasher = Keccak256::new();
        hasher.update(&public_key[1..]);

        let result = hasher.finalize();

        let address = <EthereumAddressBytes>::try_from(&result[12..]).unwrap();

        Address { address }
    }

    pub fn as_bytes(&self) -> &EthereumAddressBytes {
        &self.address
    }
    pub fn hex_address(&self) -> String {
        const HEX_LEN: usize = 40; // 20 bytes * 2
        let mut hex = String::with_capacity(HEX_LEN);
        hex.extend(hex::encode(&self.address).chars());
        hex
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_private_key_address() {
        let account = PrivateKeyAccount::from_random_private_key();
        let address: String = account.address.hex_address();

        assert_eq!(address.len(), 40);
    }

    #[test]

    fn test_random_mnemonic_address() {
        let account =
            MnemonicAccount::from_random_mnemonic(12, &"m/44'/60'/0'/0/0".parse().unwrap());
        let address: String = account.address.hex_address();

        assert_eq!(address.len(), 40);
        assert_eq!(account.mnemonic.split(" ").count(), 12);
    }
}
