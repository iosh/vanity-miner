use crate::cli;

pub struct AddressValidator {
    contains: Option<Vec<String>>,
    prefix: Option<String>,
    suffix: Option<String>,
    regex: Option<regex::Regex>,
}

impl AddressValidator {
    pub fn new(args: &cli::Args) -> AddressValidator {
        let r = args
            .match_regex
            .as_ref()
            .map(|r| regex::Regex::new(r).expect("Invalid regex pattern"));

        AddressValidator {
            contains: args.substrings.clone(),
            prefix: args.required_prefix.clone(),
            suffix: args.required_suffix.clone(),
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
            if !contains.iter().any(|c| !address.contains(c)) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cli::Args;

    #[test]
    fn test_contains_validate() {
        let validator = AddressValidator::new(&Args {
            max_attempts: None,
            max_matches: None,
            num_threads: None,
            substrings: Some(vec![
                "123".to_string(),
                "111".to_string(),
                "999999999".to_string(),
            ]),
            required_prefix: None,
            required_suffix: None,
            match_regex: None,
        });

        assert!(validator.validate("1230000000000000000000000000000000000000"));
        assert!(validator.validate("0001230000000000000000000000000000000000"));
        assert!(validator.validate("0000000000000000000000000000000000000123"));
        assert!(validator.validate("1110000000000000000000000000000000000000"));
        assert!(validator.validate("9999999990000000000000000000000000000000"));
    }

    #[test]
    fn test_prefix_validate() {
        let validator = AddressValidator::new(&Args {
            max_attempts: None,
            max_matches: None,
            num_threads: None,
            substrings: None,
            required_prefix: Some("123".to_string()),
            required_suffix: None,
            match_regex: None,
        });

        assert!(validator.validate("1230000000000000000000000000000000000000"));
        assert_eq!(
            validator.validate("0001230000000000000000000000000000000000"),
            false
        );
    }

    #[test]
    fn test_suffix_validate() {
        let validator = AddressValidator::new(&Args {
            max_attempts: None,
            max_matches: None,
            num_threads: None,
            substrings: None,
            required_prefix: None,
            required_suffix: Some("123".to_string()),
            match_regex: None,
        });

        assert!(validator.validate("0000000000000000000000000000000000000123"));
        assert_eq!(
            validator.validate("9999999990000000000000000000000000000000"),
            false
        );
    }

    #[test]
    fn test_regex_validate() {
        let validator = AddressValidator::new(&Args {
            max_attempts: None,
            max_matches: None,
            num_threads: None,
            substrings: None,
            required_prefix: None,
            required_suffix: None,
            match_regex: Some("0{10}".to_string()),
        });

        assert!(validator.validate("0000000000111111111111111111111111111111"));
        assert_eq!(
            validator.validate("1110000000111111111111111111111111111111"),
            false
        );
    }
}
