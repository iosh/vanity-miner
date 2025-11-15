use bip32::{DerivationPath, XPrv};
use bip39::Mnemonic;
use rand::RngCore;
use secp256k1::{PublicKey as SecpPubkey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

use crate::core::{
    chain::Chain,
    config::{AddressConfig, ChainConfig},
    types::{Address, KeyPair, PublicKey, Result, VanityError},
};

// Conflux address encoding constants.
const CFX_VERSION_BYTE: u8 = 0x00;
const CFX_BIT_MASK_5: u8 = 0x1F;
const CFX_BASE32_CHECKSUM_LEN: usize = 8;

const CFX_POLY_COEFFS: [u64; 5] = [
    0x98f2bc8e61,
    0x79b76d99e2,
    0xf33e5fb3c4,
    0xae2eabe2a8,
    0x1e4f43e470,
];

/// Conflux chain implementation.
pub struct ConfluxChain {
    secp: Secp256k1<secp256k1::All>,
}

impl ConfluxChain {
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
        }
    }

    fn public_key_bytes(public_key: &PublicKey) -> Result<Vec<u8>> {
        match public_key {
            PublicKey::Secp256k1(bytes) => Ok(bytes.clone()),
            _ => Err(VanityError::CryptoError(
                "Conflux only supports secp256k1 keys".into(),
            )),
        }
    }

    fn derive_pubkey_bytes(&self, secret_key: &SecretKey) -> Vec<u8> {
        SecpPubkey::from_secret_key(&self.secp, secret_key)
            .serialize_uncompressed()
            .to_vec()
    }

    fn keccak256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    fn raw20_from_public(&self, public_key: &PublicKey) -> Result<[u8; 20]> {
        let bytes = Self::public_key_bytes(public_key)?;
        debug_assert_eq!(bytes.len(), 65);
        debug_assert_eq!(bytes[0], 0x04);

        let hash = Self::keccak256(&bytes[1..]);
        Ok(hash[12..].try_into().expect("slice to 20 bytes"))
    }

    fn base32_charset() -> &'static [u8] {
        b"abcdefghjkmnprstuvwxyz0123456789"
    }

    /// Derives network prefix string from network id.
    /// 1029 -> "cfx", 1 -> "cfxtest", others -> "net{n}".
    fn network_prefix(network_id: u32) -> String {
        match network_id {
            1029 => "cfx".to_string(),
            1 => "cfxtest".to_string(),
            n => format!("net{}", n),
        }
    }

    fn encode_base32(raw20: &[u8; 20], network_id: u32) -> String {
        // normalize high nibble of the first byte to 0x1 (it means user address)
        let modified = *raw20;

        // build payload = [version byte || modified_address]
        let mut payload = Vec::with_capacity(1 + modified.len());
        payload.push(CFX_VERSION_BYTE);
        payload.extend_from_slice(&modified);

        // convert payload from 8-bit bytes to 5-bit bytes
        let payload_5_bits = Self::convert_bits_8_to_5(&payload);

        // expand prefix to 5-bite values and build checksum input
        let prefix = Self::network_prefix(network_id);
        let expanded_prefix: Vec<u8> = prefix.chars().map(|c| (c as u8) & CFX_BIT_MASK_5).collect();

        let checksum_input_len =
            expanded_prefix.len() + 1 + payload_5_bits.len() + CFX_BASE32_CHECKSUM_LEN;
        let mut checksum_input = Vec::with_capacity(checksum_input_len);
        checksum_input.extend_from_slice(&expanded_prefix);
        checksum_input.push(0);
        checksum_input.extend_from_slice(&payload_5_bits);
        checksum_input.extend_from_slice(&[0u8; CFX_BASE32_CHECKSUM_LEN]);

        let checksum = Self::calculate_checksum(&checksum_input);

        let charset = Self::base32_charset();

        let payload_str: String = payload_5_bits
            .iter()
            .map(|&b| charset[b as usize] as char)
            .collect();

        let checksum_str: String = (0..CFX_BASE32_CHECKSUM_LEN)
            .rev()
            .map(|i| {
                let index = ((checksum >> (i * 5)) & CFX_BIT_MASK_5 as u64) as usize;
                charset[index] as char
            })
            .collect();

        format!("{}:{}{}", prefix, payload_str, checksum_str)
    }

    fn convert_bits_8_to_5(data: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity((data.len() * 8 + 4) / 5);
        let mut acc: u32 = 0;
        let mut bits: u8 = 0;

        for &byte in data {
            acc = (acc << 8) | byte as u32;
            bits += 8;

            while bits >= 5 {
                bits -= 5;
                let value = ((acc >> bits) & 0x1F) as u8;
                result.push(value);
            }
        }

        if bits > 0 {
            let value = ((acc << (5 - bits)) & 0x1F) as u8;
            result.push(value);
        }

        result
    }

    fn calculate_checksum(data: &[u8]) -> u64 {
        let mut c: u64 = 1;
        for &d in data {
            let c0 = (c >> 35) as u8;
            c = ((c & 0x07ffffffff) << 5) ^ u64::from(d);

            for (i, &coeff) in CFX_POLY_COEFFS.iter().enumerate() {
                if (c0 >> i) & 1 != 0 {
                    c ^= coeff;
                }
            }
        }
        c ^ 1
    }
}

impl Chain for ConfluxChain {
    fn id(&self) -> &str {
        "conflux"
    }

    fn name(&self) -> &str {
        "Conflux"
    }

    fn generate_keypair(&self, rng: &mut dyn RngCore) -> Result<KeyPair> {
        let mut secret_bytes = [0u8; 32];

        let secret = loop {
            rng.fill_bytes(&mut secret_bytes);
            if let Ok(sec) = SecretKey::from_byte_array(&secret_bytes) {
                break sec;
            }
        };

        let public = self.derive_pubkey_bytes(&secret);

        Ok(KeyPair::Secp256k1 {
            secret: secret.secret_bytes(),
            public,
            mnemonic: None,
            derivation_path: None,
        })
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

    fn compute_address(&self, public_key: &PublicKey) -> Address {
        let raw20 = self
            .raw20_from_public(public_key)
            .expect("Conflux only supports secp256k1 public keys");
        Address {
            raw: raw20.to_vec(),
            chain_id: "conflux".to_string(),
        }
    }

    fn format_address(&self, address: &Address, config: &AddressConfig) -> String {
        let raw20: [u8; 20] = address
            .raw
            .as_slice()
            .try_into()
            .expect("Conflux address must be 20 bytes");

        match &config.chain_config {
            ChainConfig::Conflux { network_id } => Self::encode_base32(&raw20, *network_id),

            _ => panic!("conflux only supports conflux chain config"),
        }
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
                    format!("mnemonic:{}|path:{}", m.to_string(), path.to_string())
                } else {
                    hex::encode(secret)
                }
            }
            KeyPair::Ed25519 { .. } => "<unsupported>".to_string(),
        }
    }

    fn validate_address(&self, address: &str) -> bool {
        let trimmed = address.trim();
        let lower = trimmed.to_ascii_lowercase();

        // Must contain ':' separating prefix and body.
        let (prefix, body) = match lower.split_once(':') {
            Some(parts) => parts,
            None => return false,
        };

        if prefix.is_empty() {
            return false;
        }

        if body.len() <= CFX_BASE32_CHECKSUM_LEN {
            return false;
        }

        // All chars in body must be in base32 charset.
        let charset = Self::base32_charset();
        for c in body.chars() {
            if !charset.contains(&(c as u8)) {
                return false;
            }
        }

        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn base32_encoding_matches_existing_vector() {
        let mut raw: [u8; 20] = [0u8; 20];
        hex::decode_to_slice("8357E767bc8CC8f1a1ed113444661C03A89293F5", &mut raw)
            .expect("valid hex");

        let encoded = ConfluxChain::encode_base32(&raw, 1029);
        assert_eq!(encoded, "cfx:acbzt35h1wgpv6rb7yjxjvdgdub4veyx8y9hcukbdw");
    }

    #[test]
    fn base32_prefix_reflects_network_id() {
        let mut raw: [u8; 20] = [0u8; 20];
        hex::decode_to_slice("8357E767bc8CC8f1a1ed113444661C03A89293F5", &mut raw)
            .expect("valid hex");

        let mainnet = ConfluxChain::encode_base32(&raw, 1029);
        let testnet = ConfluxChain::encode_base32(&raw, 1);
        let custom = ConfluxChain::encode_base32(&raw, 123);

        assert_eq!(mainnet, "cfx:acbzt35h1wgpv6rb7yjxjvdgdub4veyx8y9hcukbdw");
        assert_eq!(
            testnet,
            "cfxtest:acbzt35h1wgpv6rb7yjxjvdgdub4veyx8yzpvatnh2"
        );
        assert_eq!(custom, "net123:acbzt35h1wgpv6rb7yjxjvdgdub4veyx8yyzbw1kj9");
    }
}
