use clap::{ArgGroup, Parser};

use crate::generator::types::AddressFormat;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(
    ArgGroup::new("use_method")
    .required(true)
    .args(&["use_mnemonic", "use_private_key"]),
))]
pub struct Args {
    /// Use a private key to generate the address.
    #[arg(long)]
    pub use_private_key: bool,

    /// Max attempts to generate addresses (default: unlimited).
    #[arg(long)]
    pub max_attempts: Option<u64>,

    /// Max matching addresses to return (default: unlimited).
    #[arg(long)]
    pub limit: Option<u64>,

    /// Number of threads to use (default: number of CPU cores).
    #[arg(long)]
    pub threads: Option<usize>,

    /// Required substring(s) in the address (case-insensitive).
    #[arg(long)]
    pub contains: Option<Vec<String>>,

    /// Required prefix for the address.
    #[arg(long)]
    pub prefix: Option<String>,

    /// Required suffix for the address.
    #[arg(long)]
    pub suffix: Option<String>,

    /// Regex for the address (Rust regex syntax).
    /// Example: "^[a-zA-Z0-9]{4}.*\\d{2}$"
    #[arg(long)]
    pub regex: Option<String>,

    /// Use a random mnemonic to generate the address.
    #[arg(long)]
    pub use_mnemonic: bool,

    /// Derivation path for mnemonic-based address generation.
    #[arg(long, default_value = "m/44'/60'/0'/0/0")]
    pub derivation_path: String,

    /// Number of words in the mnemonic (12, 15, 18, 21, or 24).
    #[arg(long, value_parser = parse_mnemonic_word_count)]
    pub mnemonic_words: Option<usize>,

    /// Address format: HEX (default) or BASE32.
    #[arg(long, default_value = "HEX")]
    pub address_format: AddressFormat,

    /// If you want to use base32 for Conflux core space, you need to specify the network id
    /// mainnet: 1029 testnet: 1028 default: 1029
    #[arg(long, default_value = "1029")]
    pub cfx_network_id: u32,
}

const MIN_MNEMONIC_WORDS: usize = 12;
const MAX_MNEMONIC_WORDS: usize = 24;

/// Validates the mnemonic word count.
fn parse_mnemonic_word_count(s: &str) -> Result<usize, String> {
    let count: usize = s
        .parse()
        .map_err(|_| "Word count must be a number".to_string())?;

    if count >= MIN_MNEMONIC_WORDS && count <= MAX_MNEMONIC_WORDS && count % 3 == 0 {
        Ok(count)
    } else {
        Err(format!(
            "Word count must be 12, 15, 18, 21, or 24. Got {}",
            count
        ))
    }
}
