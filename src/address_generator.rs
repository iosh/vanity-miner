use bip32::DerivationPath;

use crate::{
    address::{Address, MnemonicAccount, PrivateKeyAccount},
    validator::AddressValidator,
};

pub struct AddressGenerator {
    pub from_private_key: bool,
    pub derivation_path: DerivationPath,
    pub address_format: AddressFormat,
    pub validator: AddressValidator,
}

impl AddressGenerator {
    pub fn new(
        from_private_key: bool,
        derivation_path: String,
        validator: AddressValidator,
        address_format: AddressFormat,
    ) -> AddressGenerator {
        AddressGenerator {
            from_private_key,
            derivation_path: derivation_path.parse().expect("Invalid derivation path"),
            validator,
            address_format,
        }
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
        if self.from_private_key {
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

#[derive(Clone, Debug, PartialEq)]
pub enum AddressFormat {
    HEX,
    BASE32,
}

impl From<&str> for AddressFormat {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "BASE32" => AddressFormat::BASE32,
            _ => AddressFormat::HEX,
        }
    }
}
