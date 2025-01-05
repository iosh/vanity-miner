use secp256k1::rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tiny_keccak::{Hasher, Keccak};

pub struct Address {
    address: [u8; 20],
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

        let mut address = [0u8; 20];
        address.copy_from_slice(&output[12..]);

        Address { address }
    }
    pub fn display_hex_address(&self) -> String {
        format!("0x{}", hex::encode(&self.address))
    }

    pub fn hex_address(&self) -> String {
        hex::encode(&self.address)
    }
}
