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
            from_mnemonic:false,
            from_private_key:false,
            max_attempts: None,
            limit: None,
            threads: None,
            contains: Some(vec![
                "123".to_string(),
                "111".to_string(),
                "999999999".to_string(),
            ]),
            prefix: None,
            suffix: None,
            regex: None,
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
            from_mnemonic:false,
            from_private_key:false,
            max_attempts: None,
            limit: None,
            threads: None,
            contains: None,
            prefix: Some("123".to_string()),
            suffix: None,
            regex: None,
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
            from_mnemonic:false,
            from_private_key:false,
            max_attempts: None,
            limit: None,
            threads: None,
            contains: None,
            prefix: None,
            suffix: Some("123".to_string()),
            regex: None,
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
            from_mnemonic:false,
            from_private_key:false,
            max_attempts: None,
            limit: None,
            threads: None,
            contains: None,
            prefix: None,
            suffix: None,
            regex: Some("0{10}".to_string()),
        });

        assert!(validator.validate("0000000000111111111111111111111111111111"));
        assert_eq!(
            validator.validate("1110000000111111111111111111111111111111"),
            false
        );
    }
}
