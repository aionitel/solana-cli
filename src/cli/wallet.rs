use solana_sdk::signer::{keypair::Keypair, Signer};

#[derive(Debug)]
pub struct Wallet {
    pub pub_key: String,
    pub priv_key: String,
}

pub fn new_wallet() -> Wallet {
    let pair = Keypair::new();

    Wallet {
        pub_key: pair.pubkey().to_string(),
        priv_key: pair.to_base58_string(),
    }
}
