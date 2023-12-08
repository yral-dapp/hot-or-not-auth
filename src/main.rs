mod providers;

use axum::{routing::get, Json, Router};
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
use std::path::Path;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

const SERVER_PEM_FILEPATH: &str = "/home/user/.config/dfx/identity/Temp2/identity.pem";

// connect to canister & get principal_id
// generate delegated identity for user
async fn authenticate() -> Json<(String, SignedDelegation)> {
    // client identity
    let (client_secret, _client_pem) = generate_key("oauth_identity").unwrap();
    let client_identity = Secp256k1Identity::from_private_key(client_secret.clone());
    println!("client_identity: {:?}", client_identity.public_key());

    // server identity
    let path = Path::new(SERVER_PEM_FILEPATH);
    let server_identity = Secp256k1Identity::from_pem_file(path).unwrap();
    println!("server_identity: {:?}", server_identity.public_key());

    let expiration = Utc::now() + Duration::hours(1);
    let expiration = expiration.timestamp_nanos_opt().unwrap() as u64;
    let delegation = Delegation {
        pubkey: server_identity.public_key().unwrap(),
        expiration,
        targets: None,
    };

    // server identity
    let path = Path::new(SERVER_PEM_FILEPATH);
    let server_identity = Secp256k1Identity::from_pem_file(path).unwrap();
    println!("server_identity: {:?}", server_identity.public_key());
    let signature = client_identity.sign_delegation(&delegation).unwrap();
    println!("signature: {:?}", signature);

    let signed_delegation = SignedDelegation {
        delegation,
        signature: signature.signature.unwrap(),
    };
    println!("signed_delegation: {:?}", signed_delegation);

    let box_server_identity = Box::new(Secp256k1Identity::from_pem_file(path).unwrap());
    let delegated_identity = DelegatedIdentity::new(
        signature.public_key.unwrap(),
        box_server_identity,
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

    /*
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
    */
    // let delegated_pubkey = delegated_identity.public_key().unwrap();
    // let delegated_pubkey = String::from_utf8_lossy(&delegated_pubkey).to_string();
    /* {
        Ok(key) => key,
        Err(error) => format!("Error: {:?}", error),
    };*/
    Json((user_principal_id, signed_delegation))
}

fn generate_key(oauth_identity: &str) -> Result<(SecretKey, Vec<u8>), String> {
    let mnemonic = Mnemonic::new(MnemonicType::for_key_size(256).unwrap(), Language::English);
    let secret = mnemonic_to_key(&mnemonic, oauth_identity)
        .map_err(|e| format!("Converting mnemonic to key failed: {}", e))?;
    let pem = secret
        .to_sec1_pem(CRLF)
        .map_err(|e| format!("Generate Fresh Key failed: {}", e))?;
    Ok((secret, pem.as_bytes().to_vec()))
}

fn mnemonic_to_key(mnemonic: &Mnemonic, oauth_identity: &str) -> Result<SecretKey, String> {
    const DEFAULT_DERIVATION_PATH: &str = "m/44'/233'/0'/0/0";
    let path = DEFAULT_DERIVATION_PATH.parse().unwrap();
    let seed = Seed::new(mnemonic, oauth_identity);
    let pk = XPrv::derive_from_path(seed.as_bytes(), &path).map_err(|e| format!("Error: {}", e))?;
    Ok(SecretKey::from(pk.private_key()))
}

#[tokio::main]
async fn main() {
    let service = ServiceBuilder::new().layer(CorsLayer::permissive());
    let app = Router::new()
        .route("/", get(|| async { "Welcome to HotOrNot!" }))
        .route("/auth", get(authenticate))
        .layer(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
