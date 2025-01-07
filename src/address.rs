use std::fmt::Display;

use secp256k1::rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tiny_keccak::{Hasher, Keccak};

pub type EthereumAddress = [u8; 20];

pub type Keccak256Hash = [u8; 32];

pub struct Address {
    address: EthereumAddress,
}

pub struct PrivateKeyAccount {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub address: Address,
}

impl PrivateKeyAccount {
    pub fn from_random_private_key() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        PrivateKeyAccount {
            secret_key,
            public_key,
            address: Address::new(&public_key),
        }
    }
}

impl Address {
    pub fn new(public_key: &PublicKey) -> Self {
        let public_key = public_key.serialize_uncompressed();
        let public_key = &public_key[1..];
        let mut hasher = Keccak::v256();
        hasher.update(public_key);
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);

        let address = <EthereumAddress>::try_from(&output[12..]).unwrap();

        Address { address }
    }

    pub fn hex_address(&self) -> String {
        hex::encode(&self.address)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{}", hex::encode(&self.address))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_private_key_address() {
        let account = PrivateKeyAccount::from_random_private_key();
        let address: String = account.address.hex_address();

        assert_eq!(address.len(), 42);
    }
}
