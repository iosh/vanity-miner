use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["from_mnemonic", "from_private_key"]),
))]
pub struct Args {
    /// Generate address from a random mnemonic phrase.
    #[arg(long)]
    pub from_mnemonic: bool,

    /// Generate address from a private key.
    #[arg(long)]
    pub from_private_key: bool,

    /// Maximum number of attempts to generate addresses (default: unlimited).
    #[arg(short = 'a', long)]
    pub max_attempts: Option<u64>,

    /// Maximum number of matching addresses to return (default: 1).
    #[arg(short = 'l', long)]
    pub limit:  Option<u64>,

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
}
