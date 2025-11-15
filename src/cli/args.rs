use clap::{ArgGroup, Parser};

use super::validators::parse_mnemonic_word_count;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(
      group(
          ArgGroup::new("key_source")
              .required(true)
              .args(&["mnemonic", "private_key"]),
      )
  )]
pub struct Args {
    /// Target blockchain id. Default: ethereum
    #[arg(long, default_value = "ethereum")]
    pub chain: String,

    /// Use randomly generated private keys to generate addresses.
    #[arg(long = "private-key", short = 'k')]
    pub private_key: bool,

    /// Use randomly generated mnemonics to generate addresses.
    #[arg(long = "mnemonic", short = 'm')]
    pub mnemonic: bool,

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

    /// Derivation path for mnemonic-based address generation.
    #[arg(long, short = 'd', default_value = "m/44'/60'/0'/0/0")]
    pub derivation_path: String,

    /// Number of words in the mnemonic (12, 15, 18, 21, or 24).
    #[arg(long, short = 'w', value_parser = parse_mnemonic_word_count)]
    pub mnemonic_words: Option<usize>,

    /// Output CSV file path for storing found addresses and keys.
    #[arg(long = "output", short = 'o', default_value = "vanity-addresses.csv")]
    pub output: String,

    /// Also print each found address to the console.
    #[arg(long)]
    pub console: bool,

    /// Do not write results to a CSV file.
    #[arg(long = "no-file")]
    pub no_file: bool,
}
