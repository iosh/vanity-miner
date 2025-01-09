use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Maximum number of times to generate addresses.
    #[arg(short, long)]
    pub max: Option<u64>,

    /// Maximum number of results to return.
    #[arg(short, long)]
    pub limit: Option<u64>,

    /// Maximum number of concurrent threads to use.
    #[arg(short, long)]
    pub threads: Option<usize>,

    /// Strings that the address must contain.
    #[arg(short, long)]
    pub contains: Option<Vec<String>>,

    /// Prefix that the address must match.
    #[arg(short, long)]
    pub prefix: Option<String>,

    /// Suffix that the address must match.
    #[arg(short, long)]
    pub suffix: Option<String>,

    /// Regular expression that the address must match.
    #[arg(short, long)]
    pub regex: Option<String>,
}
