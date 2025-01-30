use clap::{ArgGroup, Parser};

use crate::generator::AddressFormat;

use super::validators::parse_mnemonic_word_count;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(
    ArgGroup::new("use_method")
    .required(true)
    .args(&["use_mnemonic", "use_private_key"]),
))]
pub struct Args {
    /// Use a private key to generate the address.
    #[arg(long, short = 'k')]
    pub use_private_key: bool,

    /// Max attempts to generate addresses (default: unlimited).
    #[arg(long, short = 'a')]
    pub max_attempts: Option<u64>,

    /// Max matching addresses to return (default: unlimited).
    #[arg(long, short = 'l')]
    pub limit: Option<u64>,

    /// Number of threads to use (default: number of CPU cores).
    #[arg(long, short = 't')]
    pub threads: Option<usize>,

    /// Required substring(s) in the address (case-insensitive).
    #[arg(long, short = 'c')]
    pub contains: Option<Vec<String>>,

    /// Required prefix for the address.
    #[arg(long, short = 'p')]
    pub prefix: Option<String>,

    /// Required suffix for the address.
    #[arg(long, short = 's')]
    pub suffix: Option<String>,

    /// Regex for the address (Rust regex syntax).
    /// Example: "^[a-zA-Z0-9]{4}.*\\d{2}$"
    #[arg(long, short = 'r')]
    pub regex: Option<String>,

    /// Use a random mnemonic to generate the address.
    #[arg(long, short = 'm')]
    pub use_mnemonic: bool,

    /// Derivation path for mnemonic-based address generation.
    #[arg(long, short = 'd', default_value = "m/44'/60'/0'/0/0")]
    pub derivation_path: String,

    /// Number of words in the mnemonic (12, 15, 18, 21, or 24).
    #[arg(long, short = 'w', value_parser = parse_mnemonic_word_count)]
    pub mnemonic_words: Option<usize>,

    /// Address format: HEX (default) or BASE32.
    #[arg(long, short = 'f', default_value = "HEX")]
    pub address_format: AddressFormat,

    /// Conflux network ID for address generation.
    #[arg(long, short = 'n', default_value = "1029")]
    pub cfx_network: u32,

    /// Output CSV file path for storing found addresses and keys.
    #[arg(long, short = 'o', default_value = "vanity-addresses.csv")]
    pub output_file: String,
}
