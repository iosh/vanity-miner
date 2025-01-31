use regex::Regex;

use super::{types::ValidatorType, validator::AddressValidator};

#[derive(Default)]
pub struct ValidatorBuilder {
    validators: Vec<ValidatorType>,
}

impl ValidatorBuilder {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.validators.push(ValidatorType::Prefix(prefix));
        self
    }

    pub fn with_suffix(mut self, suffix: String) -> Self {
        self.validators.push(ValidatorType::Suffix(suffix));
        self
    }

    pub fn with_contains(mut self, patterns: Vec<String>) -> Self {
        self.validators.push(ValidatorType::Contains(patterns));
        self
    }

    pub fn with_regex(mut self, pattern: String) -> Self {
        self.validators.push(ValidatorType::Regex(Box::new(
            Regex::new(&pattern).unwrap(),
        )));
        self
    }

    pub fn build(self) -> AddressValidator {
        AddressValidator::new(self.validators)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_with_prefix() {
        let validator = ValidatorBuilder::new()
            .with_prefix("123".to_string())
            .build();
        assert!(validator.validate("1230000000000000000000000000000000000000"));
    }

    #[test]
    fn test_builder_with_multiple_conditions() {
        let validator = ValidatorBuilder::new()
            .with_prefix("123".to_string())
            .with_suffix("456".to_string())
            .with_contains(vec!["789".to_string()])
            .build();
        assert!(validator.validate("1237890000000000000000000000000000000456"));
    }
}
