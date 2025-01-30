#[derive(Debug, Clone)]
pub enum ValidatorType {
    Prefix(String),
    Suffix(String),
    Contains(Vec<String>),
    Regex(String),
}

impl ValidatorType {
    pub fn validate(&self, address: &str) -> bool {
        match self {
            ValidatorType::Prefix(prefix) => address.starts_with(prefix),
            ValidatorType::Suffix(suffix) => address.ends_with(suffix),
            ValidatorType::Contains(patterns) => patterns.iter().any(|p| address.contains(p)),
            ValidatorType::Regex(pattern) => {
                if let Ok(regex) = regex::Regex::new(pattern) {
                    regex.is_match(address)
                } else {
                    false
                }
            }
        }
    }
} 