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
