use crate::validator::AddressValidator;

use super::{generator::AddressGenerator, types::AddressFormat};

pub struct GeneratorBuilder {
    use_private_key: bool,
    derivation_path: Option<String>,
    address_format: AddressFormat,
    validator: Option<AddressValidator>,
}

impl GeneratorBuilder {
    pub fn new() -> Self {
        Self {
            use_private_key: false,
            derivation_path: None,
            address_format: AddressFormat::HEX,
            validator: None,
        }
    }

    pub fn with_private_key(mut self) -> Self {
        self.use_private_key = true;
        self
    }

    pub fn with_mnemonic(mut self, path: String) -> Self {
        self.use_private_key = false;
        self.derivation_path = Some(path);
        self
    }

    pub fn with_format(mut self, format: AddressFormat) -> Self {
        self.address_format = format;
        self
    }

    pub fn with_validator(mut self, validator: AddressValidator) -> Self {
        self.validator = Some(validator);
        self
    }

    pub fn build(self) -> AddressGenerator {
        let derivation_path = self.derivation_path.unwrap_or_else(|| "m/44'/60'/0'/0/0".to_string());
        let validator = self.validator.expect("Validator is required");

        AddressGenerator::new(
            self.use_private_key,
            derivation_path,
            validator,
            self.address_format,
        )
    }
}

impl Default for GeneratorBuilder {
    fn default() -> Self {
        Self::new()
    }
} 