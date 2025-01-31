use super::types::ValidatorType;

#[derive(Debug)]
pub struct AddressValidator {
    validators: Vec<ValidatorType>,
}

impl AddressValidator {
    pub(crate) fn new(validators: Vec<ValidatorType>) -> Self {
        Self { validators }
    }

    pub fn validate(&self, address: &str) -> bool {
        self.validators.iter().all(|v| v.validate(address))
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn test_prefix_validate() {
        let validator = AddressValidator::new(vec![ValidatorType::Prefix("123".to_string())]);
        assert!(validator.validate("1230000000000000000000000000000000000000"));
        assert!(!validator.validate("0001230000000000000000000000000000000000"));
    }

    #[test]
    fn test_suffix_validate() {
        let validator = AddressValidator::new(vec![ValidatorType::Suffix("123".to_string())]);
        assert!(validator.validate("0000000000000000000000000000000000000123"));
        assert!(!validator.validate("9999999990000000000000000000000000000000"));
    }

    #[test]
    fn test_contains_validate() {
        let validator = AddressValidator::new(vec![ValidatorType::Contains(vec![
            "123".to_string(),
            "456".to_string(),
        ])]);
        assert!(validator.validate("1230000000000000000000000000000000000000"));
        assert!(validator.validate("0000000000000000000000000000000000000456"));
        assert!(!validator.validate("0000000000000000000000000000000000000789"));
    }

    #[test]
    fn test_regex_validate() {
        let validator = AddressValidator::new(vec![ValidatorType::Regex(Box::new(
            Regex::new("0{10}").unwrap(),
        ))]);
        assert!(validator.validate("0000000000111111111111111111111111111111"));
        assert!(!validator.validate("1110000000111111111111111111111111111111"));
    }

    #[test]
    fn test_multiple_validators() {
        let validator = AddressValidator::new(vec![
            ValidatorType::Prefix("123".to_string()),
            ValidatorType::Suffix("456".to_string()),
        ]);
        assert!(validator.validate("1230000000000000000000000000000000000456"));
        assert!(!validator.validate("1230000000000000000000000000000000000000"));
        assert!(!validator.validate("0000000000000000000000000000000000000456"));
    }
}
