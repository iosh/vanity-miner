pub enum ConfluxNetwork {
    Mainnet,
    Testnet,
    Net(u32),
}

impl ConfluxNetwork {
    pub fn from_u32(network: u32) -> Self {
        match network {
            1 => ConfluxNetwork::Testnet,
            1029 => ConfluxNetwork::Mainnet,
            _ => ConfluxNetwork::Net(network),
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Self::Mainnet => "cfx".to_string(),
            Self::Testnet => "cfxtest".to_string(),
            Self::Net(net) => format!("net{}", net),
        }
    }
}
