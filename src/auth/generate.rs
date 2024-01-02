use bip32::XPrv;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use k256::SecretKey;
use sec1::LineEnding::CRLF;
use tracing::log::info;

pub fn key_pair(oauth_identity: &str) -> Result<KeyPair, String> {
    let mnemonic = Mnemonic::new(MnemonicType::for_key_size(256).unwrap(), Language::English);
    let secret = mnemonic_to_key(&mnemonic, oauth_identity)
        .map_err(|e| format!("Converting mnemonic to key failed: {}", e))?;
    let public_key = secret.public_key().to_string().as_bytes().to_vec();
    let private_key = secret.to_bytes().to_vec();
    info!(
        "publen: {} , privlen: {}",
        public_key.len(),
        private_key.len()
    );
    let pem = secret
        .to_sec1_pem(CRLF)
        .map_err(|e| format!("Generate Fresh Key failed: {}", e))?;
    let pem = pem.as_str().to_owned();

    Ok(KeyPair {
        public_key,
        private_key,
        private_pem: pem,
    })
}

pub fn to_hex_string(bytes: Vec<u8>) -> String {
    bytes.iter().fold(String::new(), |mut acc, &byte| {
        acc.push_str(&format!("{:02x}", byte));
        acc
    })
}

fn mnemonic_to_key(mnemonic: &Mnemonic, oauth_identity: &str) -> Result<SecretKey, String> {
    const DEFAULT_DERIVATION_PATH: &str = "m/44'/233'/0'/0/0";
    let path = DEFAULT_DERIVATION_PATH.parse().unwrap();
    let seed = Seed::new(mnemonic, oauth_identity);
    let pk = XPrv::derive_from_path(seed.as_bytes(), &path).map_err(|e| format!("Error: {}", e))?;
    Ok(SecretKey::from(pk.private_key()))
}

#[derive(Default, Clone)]
pub struct KeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub private_pem: String,
}
