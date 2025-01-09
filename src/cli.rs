use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Maximum number of attempts to generate addresses.
    #[arg(short = 'a', long)]
    pub max_attempts: Option<u64>,

    /// Maximum number of matching addresses to return.
    #[arg(short = 'm', long)]
    pub max_matches: Option<u64>,

    /// Number of concurrent threads to use.
    #[arg(short = 't', long)]
    pub num_threads: Option<usize>,

    /// Substrings that the address must contain.
    #[arg(short = 'c', long)]
    pub substrings: Option<Vec<String>>,

    /// Prefix that the address must match.
    #[arg(short = 'p', long)]
    pub required_prefix: Option<String>,

    /// Suffix that the address must match.
    #[arg(short = 's', long)]
    pub required_suffix: Option<String>,

    /// Regular expression that the address must match.
    #[arg(short = 'r', long)]
    pub match_regex: Option<String>,
}
