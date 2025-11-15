use bip32::DerivationPath;
use bip39::Mnemonic;
use rand::RngCore;

use super::config::AddressConfig;
use super::types::{Address, KeyPair, PublicKey, Result};

pub trait Chain: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;

    fn generate_keypair(&self, rng: &mut dyn RngCore) -> Result<KeyPair>;

    fn derive_from_mnemonic(&self, mnemonic: &Mnemonic, path: &DerivationPath) -> Result<KeyPair>;

    fn keypair_from_secret(&self, secret: &[u8]) -> Result<KeyPair>;

    fn compute_address(&self, public_key: &PublicKey) -> Address;

    fn format_address(&self, address: &Address, config: &AddressConfig) -> String;

    fn format_secret(&self, keypair: &KeyPair) -> String;

    fn validate_address(&self, address: &str) -> bool;
}
