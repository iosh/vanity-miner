use clap::{ArgGroup, Parser};

use crate::address_generator::AddressFormat;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["from_mnemonic", "from_private_key"]),
))]
pub struct Args {
    /// Generate address from a private key.
    #[arg(long)]
    pub from_private_key: bool,

    /// Maximum number of attempts to generate addresses (default: unlimited).
    #[arg(short = 'a', long)]
    pub max_attempts: Option<u64>,

    /// Maximum number of matching addresses to return (default: 1).
    #[arg(short = 'l', long)]
    pub limit: Option<u64>,

    /// Number of concurrent threads to use (default: number of CPU cores).
    #[arg(short = 't', long)]
    pub threads: Option<usize>,

    /// Substrings that the address must contain (case-insensitive).
    #[arg(short = 'c', long)]
    pub contains: Option<Vec<String>>,

    /// Prefix that the address must start with.
    #[arg(short = 'p', long)]
    pub prefix: Option<String>,

    /// Suffix that the address must end with.
    #[arg(short = 's', long)]
    pub suffix: Option<String>,

    /// Regular expression that the address must match (supports Rust regex syntax).
    /// Example: "^[a-zA-Z0-9]{4}.*\\d{2}$"
    #[arg(short = 'r', long)]
    pub regex: Option<String>,

    /// Generate address from a random mnemonic phrase.
    #[arg(long)]
    pub from_mnemonic: bool,

    /// Derivation path to use when generating a mnemonic phrase address.
    #[arg(long, default_value = "m/44'/60'/0'/0/0")]
    pub derivation_path: String,

    /// Number of mnemonic words to use when generating a mnemonic phrase address.
    #[arg(short, long, value_parser = parse_word_count)]
    pub mnemonic_words: Option<usize>,

    /// Address format HEX(Ethereum Conflux eSpace) or BASE32 (Conflux core space) default HEX
    #[arg(long, default_value = "HEX")]
    pub address_format: AddressFormat,

    /// If you want to use base32 for Conflux core space, you need to specify the network id
    /// mainnet: 1029 testnet: 1028 default: 1029
    #[arg(long, default_value = "1029")]
    pub cfx_network: u32,
}

const MIN_NB_WORDS: usize = 12;
const MAX_NB_WORDS: usize = 24;

fn parse_word_count(word_count: &str) -> Result<usize, String> {
    let word_count: usize = word_count
        .parse()
        .map_err(|_| "Word count must be a number".to_string())?;

    if word_count < MIN_NB_WORDS || word_count % 3 != 0 || word_count > MAX_NB_WORDS {
        Err(format!(
            "Word count must be between {} and {}, and must be a multiple of 3",
            MIN_NB_WORDS, MAX_NB_WORDS
        ))
    } else {
        Ok(word_count)
    }
}
