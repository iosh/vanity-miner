use bip32::DerivationPath;

use crate::{
    address::{
        account::{MnemonicAccount, PrivateKeyAccount},
        encoding::Address,
    },
    validator::AddressValidator,
};

use super::types::AddressFormat;

pub struct AddressGenerator {
    use_private_key: bool,
    derivation_path: DerivationPath,
    address_format: AddressFormat,
    validator: AddressValidator,
}

impl AddressGenerator {
    // Constructor is now primarily used by the builder
    pub(crate) fn new(
        use_private_key: bool,
        derivation_path: String,
        validator: AddressValidator,
        address_format: AddressFormat,
    ) -> AddressGenerator {
        AddressGenerator {
            use_private_key,
            derivation_path: derivation_path.parse().expect("Invalid derivation path"),
            validator,
            address_format,
        }
    }

    // Static methods to create builder
    pub fn private_key() -> super::builder::GeneratorBuilder {
        super::builder::GeneratorBuilder::new().with_private_key()
    }

    pub fn mnemonic(path: String) -> super::builder::GeneratorBuilder {
        super::builder::GeneratorBuilder::new().with_mnemonic(path)
    }

    pub fn new_random_address(&self, net: u32) -> Option<(String, String)> {
        let (address, secret) = self.generate_address_and_secret(net);

        let address_to_validate = match self.address_format {
            AddressFormat::HEX => &address,
            AddressFormat::BASE32 => address.split(':').nth(1).expect("Invalid base32 address"),
        };

        if self.validator.validate(address_to_validate) {
            let formatted_address = match self.address_format {
                AddressFormat::HEX => format!("0x{}", address),
                AddressFormat::BASE32 => address,
            };
            return Some((formatted_address, secret));
        };
        None
    }

    fn generate_address_and_secret(&self, net: u32) -> (String, String) {
        if self.use_private_key {
            let private_key_account = PrivateKeyAccount::from_random_private_key();
            let address = self.format_address(&private_key_account.address, net);
            (
                address,
                private_key_account.secret_key.display_secret().to_string(),
            )
        } else {
            let mnemonic_account = MnemonicAccount::from_random_mnemonic(12, &self.derivation_path);
            let address = self.format_address(&mnemonic_account.address, net);
            (address, mnemonic_account.mnemonic.to_string())
        }
    }

    fn format_address(&self, address: &Address, net: u32) -> String {
        match self.address_format {
            AddressFormat::HEX => address.hex_address(),
            AddressFormat::BASE32 => address.base32_address(net),
        }
    }
}
