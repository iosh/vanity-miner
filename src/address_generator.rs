use bip32::DerivationPath;

use crate::{
    address::{MnemonicAccount, PrivateKeyAccount},
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
            derivation_path: derivation_path
                .parse()
                .map_err(|e| format!("Invalid derivation path: {}", e))
                .unwrap(),
            validator,
            address_format,
        }
    }

    pub fn new_random_address(&self, net: u32) -> Option<(String, String)> {
        let (address, secret) = if self.from_private_key {
            let private_key_account = PrivateKeyAccount::from_random_private_key();
            let address = match self.address_format {
                AddressFormat::HEX => private_key_account.address.hex_address(),
                AddressFormat::BASE32 => private_key_account.address.base32_address(net),
            };
            (
                address,
                private_key_account.secret_key.display_secret().to_string(),
            )
        } else {
            let mnemonic_account = MnemonicAccount::from_random_mnemonic(12, &self.derivation_path);

            let address = match self.address_format {
                AddressFormat::HEX => mnemonic_account.address.hex_address(),
                AddressFormat::BASE32 => mnemonic_account.address.base32_address(net),
            };
            (address, mnemonic_account.mnemonic.to_string())
        };

        let match_address = match self.address_format {
            AddressFormat::HEX => &address,
            AddressFormat::BASE32 => {
                let parts: Vec<&str> = address.split(':').collect();
                if parts.len() > 1 {
                    parts[1]
                } else {
                    panic!("Invalid address format")
                }
            }
        };

        if self.validator.validate(match_address) {
            return match self.address_format {
                AddressFormat::HEX => Some((format!("0x{}", address), secret)),
                AddressFormat::BASE32 => Some((address, secret)),
            };
        };
        None
    }
}

#[derive(Clone, Debug)]
pub enum AddressFormat {
    HEX,
    BASE32,
}

impl From<&str> for AddressFormat {
    fn from(s: &str) -> Self {
        match s {
            "HEX" => AddressFormat::HEX,
            "BASE32" => AddressFormat::BASE32,
            _ => panic!("Invalid address format"),
        }
    }
}
