use secp256k1::rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1};
use tiny_keccak::{Hasher, Keccak};

fn main() {
    miner_address();
}

fn miner_address() {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

    let address = public_key_to_address(&public_key);
    println!("address: {}", address);
    println!("secret_key: {}", secret_key.display_secret());
}

fn public_key_to_address(public_key: &PublicKey) -> String {
    let public_key = public_key.serialize_uncompressed();

    let public_key = &public_key[1..];

    let mut hasher = Keccak::v256();
    hasher.update(public_key);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    let address = hex::encode(&output[12..]);

    format!("0x{}", address)
}
