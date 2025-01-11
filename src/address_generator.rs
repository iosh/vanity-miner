use bip32::DerivationPath;

use crate::{
    address::{MnemonicAccount, PrivateKeyAccount},
    validator::AddressValidator,
};

pub struct AddressGenerator {
    pub from_private_key: bool,
    pub derivation_path: DerivationPath,

    pub validator: AddressValidator,
}

impl AddressGenerator {
    pub fn new(
        from_private_key: bool,
        derivation_path: String,
        validator: AddressValidator,
    ) -> AddressGenerator {
        AddressGenerator {
            from_private_key,
            derivation_path: derivation_path
                .parse()
                .map_err(|e| format!("Invalid derivation path: {}", e))
                .unwrap(),
            validator,
        }
    }

    pub fn new_random_address(&self) -> Option<(String, String)> {
        let (address, secret) = if self.from_private_key {
            let private_key_account = PrivateKeyAccount::from_random_private_key();
            (
                private_key_account.address.hex_address(),
                private_key_account.secret_key.display_secret().to_string(),
            )
        } else {
            let mnemonic_account = MnemonicAccount::from_random_mnemonic(12, &self.derivation_path);
            (
                mnemonic_account.address.hex_address(),
                mnemonic_account.mnemonic.to_string(),
            )
        };

        if self.validator.validate(&address) {
            return Some((format!("0x{}", address), secret));
        };
        None
    }
}
