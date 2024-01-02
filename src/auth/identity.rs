use super::agent_js;
use super::generate;
use axum::extract::State;
use axum::Json;
use candid::{Decode, Encode};
use chrono::{Duration, Utc};
use ic_agent::{
    export::Principal,
    identity::{DelegatedIdentity, Delegation, Secp256k1Identity, SignedDelegation},
    Agent, Identity,
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::SystemTime,
};
use tracing::log::info;

pub async fn authenticate(
    identity_keeper: State<Arc<RwLock<IdentityKeeper>>>,
    oauth_identity: String,
) -> Json<(String, agent_js::DelegationIdentity, String)> {
    // client identity
    let client_pem: Option<generate::KeyPair> = {
        let read_access = identity_keeper.read().unwrap();
        read_access.oauth_map.get(&oauth_identity).cloned()
    };
    let client_pem = client_pem.unwrap_or_else(|| {
        let new_client_pem = generate::key_pair(&oauth_identity).unwrap();
        {
            let mut write_access = identity_keeper.write().unwrap();
            write_access
                .oauth_map
                .insert(oauth_identity.to_owned(), new_client_pem.clone());
        }
        new_client_pem
    });
    let client_identity = Secp256k1Identity::from_pem(client_pem.private_pem.as_bytes()).unwrap();

    // create Temp session
    let client_temp_session_identifier = format!("{}, {:?}", oauth_identity, SystemTime::now());
    let client_temp_pem = generate::key_pair(&client_temp_session_identifier).unwrap();
    let client_temp_identity =
        Secp256k1Identity::from_pem(client_temp_pem.private_pem.as_bytes()).unwrap();

    let expiration = Utc::now() + Duration::hours(12);
    let expiration = expiration.timestamp_nanos_opt().unwrap().unsigned_abs();

    // delegation
    let delegation = Delegation {
        pubkey: client_temp_identity.public_key().unwrap(),
        expiration,
        targets: None,
    };

    let signature = client_identity.sign_delegation(&delegation).unwrap();
    info!("signature: {:?}", signature);
    info!("Expiration: {}", delegation.expiration);

    let signed_delegation = SignedDelegation {
        delegation,
        signature: signature.signature.unwrap(),
    };
    info!("signed_delegation: {:?}", signed_delegation);

    let signature_pubkey = signature.public_key.unwrap();
    let delegated_identity = DelegatedIdentity::new(
        signature_pubkey.clone(),
        Box::new(client_temp_identity.clone()),
        vec![signed_delegation.clone()],
    );
    info!("{}", client_identity.sender().unwrap());
    info!("{}", delegated_identity.sender().unwrap());
    let sender_principal = delegated_identity.sender().unwrap().to_text();

    let inner_pubkey = client_temp_identity.public_key().unwrap();
    let inner_private = client_temp_pem.private_key.clone();

    let shareable_delegated_identity = agent_js::DelegationIdentity {
        _inner: vec![inner_pubkey, inner_private],
        _delegation: agent_js::DelegationChain {
            delegations: vec![agent_js::SignedDelegation {
                delegation: agent_js::Delegation {
                    expiration: generate::to_hex_string(
                        signed_delegation
                            .delegation
                            .expiration
                            .to_be_bytes()
                            .to_vec(),
                    ),
                    pubkey: signed_delegation.delegation.pubkey,
                    targets: None,
                },
                signature: signed_delegation.signature,
            }],
            public_key: signature_pubkey.clone(),
        },
    };

    let agent_with_client_identity = Agent::builder()
        .with_verify_query_signatures(false)
        //.with_url("https://ic0.app")
        .with_url("http://127.0.0.1:4943")
        .with_identity(client_identity)
        .build()
        .unwrap();

    agent_with_client_identity.fetch_root_key().await.unwrap();
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
    // agent_with_delegated_identity.get_principal()
    let delegated_result = match agent_with_delegated_identity
        .query(&canister_id, "get_principal_id")
        .with_arg(Encode!().unwrap())
        .call()
        .await
    {
        Ok(resp) => Decode!(resp.as_slice(), String).unwrap(),
        Err(error) => error.to_string(),
    };

    info!("Delegated Principal: {}", delegated_result);
    info!(
        "LEN compare: {} : {}",
        client_temp_pem.public_key.len(),
        client_temp_identity.public_key().unwrap().len()
    );

    Json((
        user_principal_id,
        shareable_delegated_identity,
        sender_principal,
    ))
}

pub struct IdentityKeeper {
    pub oauth_map: HashMap<String, generate::KeyPair>,
}
