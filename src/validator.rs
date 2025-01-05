use crate::args;

#[derive(Debug, Clone)]
pub struct FoundAddress {
    pub address: String,
}

pub struct AddressValidator {
    contains: Option<Vec<String>>,
    prefix: Option<String>,
    suffix: Option<String>,
    regex: Option<regex::Regex>,
}

impl AddressValidator {
    pub fn new(args: &args::Args) -> AddressValidator {
        let r = args
            .regex
            .as_ref()
            .map(|r| regex::Regex::new(r).expect("Invalid regex pattern"));

        AddressValidator {
            contains: args.contains.clone(),
            prefix: args.prefix.clone(),
            suffix: args.suffix.clone(),
            regex: r,
        }
    }

    pub fn validate(&self, address: &str) -> bool {
        if let Some(prefix) = &self.prefix {
            if !address.starts_with(prefix) {
                return false;
            }
        }

        if let Some(suffix) = &self.suffix {
            if !address.ends_with(suffix) {
                return false;
            }
        }

        if let Some(regex) = &self.regex {
            if !regex.is_match(address) {
                return false;
            }
        }

        if let Some(contains) = &self.contains {
            if contains.iter().any(|c| !address.contains(c)) {
                return false;
            }
        }

        true
    }
}
