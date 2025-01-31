use bip32::{DerivationPath, XPrv};
use bip39::Mnemonic;
use secp256k1::rand::rngs::ThreadRng;
use secp256k1::{generate_keypair, SecretKey};

use super::encoding::Address;

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

        let private_key = XPrv::derive_from_path(&seed, path).unwrap();

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
        let (secret_key, public_key) = generate_keypair(&mut ThreadRng::default());

        PrivateKeyAccount {
            secret_key,
            address: Address::from_public_key(&public_key.serialize_uncompressed()),
        }
    }
}
