use sha3::{Digest, Keccak256};

pub type EthereumAddressBytes = [u8; 20];

pub struct Address {
    address: EthereumAddressBytes,
}

const CHARSET: &[u8] = b"abcdefghjkmnprstuvwxyz0123456789";
const BIT_MASK_5: u8 = 0x1f;
const BASE32_CHECKSUM_LEN: usize = 8;

impl Address {
    pub fn from_public_key(public_key: &[u8]) -> Self {
        let mut hasher = Keccak256::new();
        hasher.update(&public_key[1..]);

        let result = hasher.finalize();

        let address = <EthereumAddressBytes>::try_from(&result[12..]).unwrap();

        Address { address }
    }
    pub fn hex_address(&self) -> String {
        format!("{}", hex::encode(&self.address))
    }

    pub fn base32_address(&self, network: u32) -> String {
        const VERSION_BYTE: u8 = 0x00;
        const HIGH_NIBBLE_MASK: u8 = 0xF0;
        const LOW_NIBBLE_MASK: u8 = 0x0F;
        const FIXED_HIGH_NIBBLE: u8 = 0x10;

        let mut modified_address = self.address;

        // Change the first hex to 1.
        let high_nibble = modified_address[0] & HIGH_NIBBLE_MASK;
        if high_nibble != FIXED_HIGH_NIBBLE {
            modified_address[0] = (modified_address[0] & LOW_NIBBLE_MASK) | FIXED_HIGH_NIBBLE;
        }

        // 确定前缀
        let prefix = match network {
            1 => "cfxtest",
            1029 => "cfx",
            _ => &format!("net{}", network),
        };

        let mut payload = Vec::with_capacity(1 + modified_address.len());
        payload.push(VERSION_BYTE);
        payload.extend_from_slice(&modified_address);

        let payload_5_bits = convert_bits_8_to_5(&payload);

        let expanded_prefix: Vec<u8> = prefix.chars().map(|c| (c as u8) & BIT_MASK_5).collect();

        let checksum_input_len =
            expanded_prefix.len() + 1 + payload_5_bits.len() + BASE32_CHECKSUM_LEN;
        let mut checksum_input = Vec::with_capacity(checksum_input_len);
        checksum_input.extend_from_slice(&expanded_prefix);
        checksum_input.push(0);
        checksum_input.extend_from_slice(&payload_5_bits);
        checksum_input.extend_from_slice(&[0; BASE32_CHECKSUM_LEN]);

        let checksum = calculate_checksum(&checksum_input);

        let payload_str: String = payload_5_bits
            .iter()
            .map(|&b| CHARSET[b as usize] as char)
            .collect();

        let checksum_str: String = (0..BASE32_CHECKSUM_LEN)
            .rev()
            .map(|i| {
                let index = ((checksum >> (i * 5)) & BIT_MASK_5 as u64) as usize;
                CHARSET[index] as char
            })
            .collect();

        format!("{}:{}{}", prefix, payload_str, checksum_str)
    }
}

fn convert_bits_8_to_5(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len() * 8 + 5 - 1);
    let mut bit_accumulator: u16 = 0;
    let mut num_bits_in_accumulator: u8 = 0;
    let groupmask = (1 << 5) - 1;

    for &byte in data.iter() {
        bit_accumulator = (bit_accumulator << 8) | u16::from(byte);
        num_bits_in_accumulator += 8;

        while num_bits_in_accumulator >= 5 {
            result.push((bit_accumulator >> (num_bits_in_accumulator - 5)) as u8);
            bit_accumulator &= !(groupmask << (num_bits_in_accumulator - 5));
            num_bits_in_accumulator -= 5;
        }
    }

    if num_bits_in_accumulator > 0 {
        result.push((bit_accumulator << (5 - num_bits_in_accumulator)) as u8);
    }

    result
}
const POLY_COEFFS: [u64; 5] = [
    0x98f2bc8e61,
    0x79b76d99e2,
    0xf33e5fb3c4,
    0xae2eabe2a8,
    0x1e4f43e470,
];

fn calculate_checksum(data: &[u8]) -> u64 {
    let mut c: u64 = 1;
    for &d in data {
        let c0 = (c >> 35) as u8;
        c = ((c & 0x07ffffffff) << 5) ^ u64::from(d);

        for (i, &coeff) in POLY_COEFFS.iter().enumerate() {
            if (c0 >> i) & 1 != 0 {
                c ^= coeff;
            }
        }
    }
    c ^ 1
}
#[cfg(test)]
mod tests {
    use crate::address::account::{MnemonicAccount, PrivateKeyAccount};

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
        let mut addr: [u8; 20] = [0; 20];
        hex::decode_to_slice("8357E767bc8CC8f1a1ed113444661C03A89293F5", &mut addr).unwrap();

        let address = Address { address: addr };
        assert_eq!(
            address.base32_address(1029),
            "cfx:aakzt35h1wgpv6rb7yjxjvdgdub4veyx8ypbtpye6b"
        )
    }
}
