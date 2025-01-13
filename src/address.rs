use bip32::{DerivationPath, XPrv};
use bip39::Mnemonic;
use secp256k1::rand::rngs::ThreadRng;
use secp256k1::{Secp256k1, SecretKey};
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

const CHARSET: &[u8] = b"abcdefghjkmnprstuvwxyz0123456789";

impl Address {
    pub fn from_public_key(public_key: &[u8]) -> Self {
        let mut hasher = Keccak256::new();
        hasher.update(&public_key[1..]);

        let result = hasher.finalize();

        let address = <EthereumAddressBytes>::try_from(&result[12..]).unwrap();

        Address { address }
    }
    pub fn hex_address(&self) -> String {
        const HEX_LEN: usize = 40; // 20 bytes * 2
        let mut hex = String::with_capacity(HEX_LEN);
        hex.extend(hex::encode(&self.address).chars());
        hex
    }
    pub fn base32_address(&self, network: u32) -> String {
        let version_byte = 0x00;
        let prefix: String = match network {
            1 => "cfxtest".to_string(),
            1029 => "cfx".to_string(),
            _ => format!("net{}", network),
        };

        // 构建 payload
        let mut payload = Vec::with_capacity(1 + self.address.len());
        payload.push(version_byte);
        payload.extend_from_slice(&self.address);

        // 将 payload 从 8 位转换为 5 位
        let payload_5_bits = convert_bits_8_to_5(&payload);

        // 构建校验和输入数据
        let expanded_prefix: Vec<u8> = prefix.chars().map(|c| (c as u8) & 0x1f).collect();
        let checksum_input = expanded_prefix
            .iter()
            .chain(std::iter::once(&0))
            .chain(payload_5_bits.iter())
            .chain([0; 8].iter())
            .cloned()
            .collect::<Vec<u8>>();

        // 计算校验和
        let checksum = calculate_checksum(&checksum_input);

        // 将 5 位 payload 和校验和转换为字符串
        let payload_str = payload_5_bits
            .iter()
            .map(|&b| CHARSET[b as usize] as char)
            .collect::<String>();
        let checksum_str: String = (0..8)
            .rev()
            .map(|i| CHARSET[((checksum >> (i * 5)) & 31) as usize] as char)
            .collect();

        // 拼接最终地址字符串
        format!("{}:{}{}", prefix, payload_str, checksum_str)
    }
}

fn convert_bits_8_to_5(data: &[u8]) -> Vec<u8> {
    let mut bit_accumulator: u16 = 0;
    let mut num_bits_in_accumulator: u8 = 0;
    let mut result = Vec::new();

    for byte in data.iter() {
        bit_accumulator = (bit_accumulator << 8) | u16::from(*byte);
        num_bits_in_accumulator += 8;

        while num_bits_in_accumulator >= 5 {
            result.push((bit_accumulator >> (num_bits_in_accumulator - 5)) as u8 & 0x1f);
            bit_accumulator &= (1 << (num_bits_in_accumulator - 5)) - 1;
            num_bits_in_accumulator -= 5;
        }
    }

    if num_bits_in_accumulator > 0 {
        result.push((bit_accumulator << (5 - num_bits_in_accumulator)) as u8 & 0x1f);
    }

    result
}

fn calculate_checksum(data: &[u8]) -> u64 {
    let mut c: u64 = 1;
    let poly_coeffs: [u64; 5] = [
        0x98f2bc8e61,
        0x79b76d99e2,
        0xf33e5fb3c4,
        0xae2eabe2a8,
        0x1e4f43e470,
    ];
    for &d in data {
        let c0 = (c >> 35) as u8;
        c = ((c & 0x07ffffffff) << 5) ^ u64::from(d);

        for (i, &coeff) in poly_coeffs.iter().enumerate() {
            if (c0 >> i) & 1 != 0 {
                c ^= coeff;
            }
        }
    }
    c ^ 1
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

    #[test]
    fn test_base32_address_encode() {
        let add = hex::decode("1357E767bc8CC8f1a1ed113444661C03A89293F5").unwrap();

        let address = Address {
            address: add.try_into().unwrap(),
        };
        assert_eq!(
            address.base32_address(1),
            "cfxtest:aakzt35h1wgpv6rb7yjxjvdgdub4veyx8ygm66ws2n"
        )
    }
}
