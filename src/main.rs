mod providers;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use bip32::XPrv;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use candid::{Decode, Encode};
use chrono::{Duration, Utc};
use ic_agent::{
    export::Principal,
    identity::{DelegatedIdentity, Delegation, Secp256k1Identity, SignedDelegation},
    Agent, Identity,
};
use k256::SecretKey;
use sec1::LineEnding::CRLF;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::SystemTime,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

// connect to canister & get principal_id
// generate delegated identity for user
async fn authenticate(
    identity_keeper: State<Arc<RwLock<IdentityKeeper>>>,
    oauth_identity: String,
) -> Json<(String, SignedDelegation, Vec<u8>, Vec<u8>, Vec<u8>)> {
    // client identity
    let client_pem: Option<KeyPair> = {
        let read_access = identity_keeper.read().unwrap();
        read_access.oauth_map.get(&oauth_identity).cloned()
    };
    let client_pem = client_pem.unwrap_or_else(|| {
        let new_client_pem = generate_key(&oauth_identity).unwrap();
        {
            let mut write_access = identity_keeper.write().unwrap();
            write_access
                .oauth_map
                .insert(oauth_identity.to_owned(), new_client_pem.clone());
        }
        new_client_pem
    });
    let client_identity = Secp256k1Identity::from_pem(client_pem.private_pem.as_bytes()).unwrap();
    let client_pubkey_len = client_identity.public_key().unwrap().len();
    println!("client_pubkey_len: {}", client_pubkey_len,);

    // create Temp session
    let client_temp_session_identifier = format!("{}, {:?}", oauth_identity, SystemTime::now());
    let client_temp_pem = generate_key(&client_temp_session_identifier).unwrap();
    let client_temp_identity =
        Secp256k1Identity::from_pem(client_temp_pem.private_pem.as_bytes()).unwrap();

    let expiration = Utc::now() + Duration::hours(1);
    let expiration = expiration.timestamp_nanos_opt().unwrap().unsigned_abs();

    // delegation
    let delegation = Delegation {
        pubkey: client_temp_identity.public_key().unwrap(),
        expiration,
        targets: None,
    };

    let signature = client_identity.sign_delegation(&delegation).unwrap();
    println!("signature: {:?}", signature);
    println!("Expiration: {}", delegation.expiration);

    let signed_delegation = SignedDelegation {
        delegation,
        signature: signature.signature.unwrap(),
    };
    println!("signed_delegation: {:?}", signed_delegation);

    let signature_pubkey = signature.public_key.unwrap();
    let delegated_identity = DelegatedIdentity::new(
        signature_pubkey.clone(),
        Box::new(client_temp_identity.clone()),
        vec![signed_delegation.clone()],
    );

    let agent_with_client_identity = Agent::builder()
        .with_verify_query_signatures(false)
        //.with_url("https://ic0.app")
        .with_url("http://127.0.0.1:4943")
        .with_identity(client_identity)
        .build()
        .unwrap();

    agent_with_client_identity.fetch_root_key().await.unwrap();
    // canister indexer?
    let canister_id = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();

    let user_principal_id = match agent_with_client_identity
        .query(&canister_id, "get_principal_id")
        .with_arg(Encode!().unwrap())
        .call()
        .await
    {
        Ok(resp) => Decode!(resp.as_slice(), String).unwrap(),
        Err(error) => error.to_string(),
    };

    let agent_with_delegated_identity = Agent::builder()
        .with_verify_query_signatures(false)
        .with_url("http://127.0.0.1:4943")
        .with_identity(delegated_identity)
        .build()
        .unwrap();
    agent_with_delegated_identity
        .fetch_root_key()
        .await
        .unwrap();
    let delegated_result = match agent_with_delegated_identity
        .query(&canister_id, "get_principal_id")
        .with_arg(Encode!().unwrap())
        .call()
        .await
    {
        Ok(resp) => Decode!(resp.as_slice(), String).unwrap(),
        Err(error) => error.to_string(),
    };

    println!("Delegated Principal: {}", delegated_result);
    println!(
        "LEN compare: {} : {}",
        client_temp_pem.public_key.len(),
        client_temp_identity.public_key().unwrap().len()
    );

    Json((
        user_principal_id,
        signed_delegation,
        signature_pubkey,
        client_temp_pem.public_key,
        client_temp_pem.private_key,
    ))
}

#[derive(Default, Clone)]
struct KeyPair {
    public_key: Vec<u8>,
    private_key: Vec<u8>,
    private_pem: String,
}

fn generate_key(oauth_identity: &str) -> Result<KeyPair, String> {
    let mnemonic = Mnemonic::new(MnemonicType::for_key_size(256).unwrap(), Language::English);
    let secret = mnemonic_to_key(&mnemonic, oauth_identity)
        .map_err(|e| format!("Converting mnemonic to key failed: {}", e))?;
    let public_key = secret.public_key().to_string().as_bytes().to_vec();
    let private_key = secret.to_bytes().to_vec();
    println!(
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

fn mnemonic_to_key(mnemonic: &Mnemonic, oauth_identity: &str) -> Result<SecretKey, String> {
    const DEFAULT_DERIVATION_PATH: &str = "m/44'/233'/0'/0/0";
    let path = DEFAULT_DERIVATION_PATH.parse().unwrap();
    let seed = Seed::new(mnemonic, oauth_identity);
    let pk = XPrv::derive_from_path(seed.as_bytes(), &path).map_err(|e| format!("Error: {}", e))?;
    Ok(SecretKey::from(pk.private_key()))
}

struct IdentityKeeper {
    oauth_map: HashMap<String, KeyPair>,
}

#[tokio::main]
async fn main() {
    let identity_keeper = IdentityKeeper {
        oauth_map: HashMap::new(),
    };
    let identity_keeper: Arc<RwLock<IdentityKeeper>> = Arc::new(RwLock::new(identity_keeper));
    let service = ServiceBuilder::new().layer(CorsLayer::permissive());
    let app = Router::new()
        .route("/", get(|| async { "Welcome to HotOrNot!" }))
        .route("/auth", post(authenticate))
        .layer(service)
        .with_state(identity_keeper);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
