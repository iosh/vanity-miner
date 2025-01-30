pub enum ConfluxNetwork {
    Mainnet,
    Testnet,
    Net(u32),
}

impl ConfluxNetwork {
    pub fn from_str(network: &str) -> Self {
        match network {
            "cfx" => Self::Mainnet,
            "cfxtest" => Self::Testnet,
            _ => Self::Net(network.parse().unwrap()),
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
