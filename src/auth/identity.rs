use super::agent_js;
use super::generate;
use axum::{
    extract::{FromRef, State},
    Json,
};
use axum_extra::extract::cookie::{Cookie, Key, SignedCookieJar};
use chrono::{Duration, Utc};
use ic_agent::{
    identity::{DelegatedIdentity, Delegation, Secp256k1Identity, SignedDelegation},
    Identity,
};
use leptos::*;
use leptos_router::RouteListing;
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tracing::log::info;

pub async fn generate_session(
    identity_keeper: State<IdentityKeeper>,
    mut jar: SignedCookieJar,
) -> (SignedCookieJar, Json<SessionResponse>) {
    let user_identity: Option<String> = match jar.get("user_identity") {
        Some(val) => Some(val.value().to_owned()),
        None => None,
    };
    info!("User check: {:?}", user_identity);
    // client identity
    let user_key_pair: Option<generate::KeyPair> = if user_identity.is_none() {
        None
    } else {
        let read_access = identity_keeper.oauth_map.read().await;
        read_access.get(&user_identity.unwrap()).cloned()
    };
    let user_key_pair = match user_key_pair {
        Some(kp) => kp,
        None => {
            let new_key_pair = generate::key_pair().unwrap();
            {
                let write_access = identity_keeper.oauth_map.write();
                write_access
                    .await
                    .insert(new_key_pair.public_key.to_owned(), new_key_pair.clone());
            }
            new_key_pair
        }
    };
    let client_identity =
        Secp256k1Identity::from_pem(user_key_pair.private_pem.as_bytes()).unwrap();

    // create Temp session
    let client_temp_pem = generate::key_pair().unwrap();
    let client_temp_identity =
        Secp256k1Identity::from_pem(client_temp_pem.private_pem.as_bytes()).unwrap();

    let expiration = Utc::now() + Duration::days(30);
    let expiration = expiration.timestamp_nanos_opt().unwrap().unsigned_abs();

    // delegation
    let delegation = Delegation {
        pubkey: client_temp_identity.public_key().unwrap(),
        expiration,
        targets: None,
    };

    let signature = client_identity.sign_delegation(&delegation).unwrap();
    info!("signature: {:?}", signature);
    info!("expiration: {}", delegation.expiration);

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
    // let sender_principal = delegated_identity.sender().unwrap().to_text();

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
    let session_response = SessionResponse {
        user_identity: user_key_pair.public_key.to_owned(),
        delegation_identity: shareable_delegated_identity,
    };

    info!("{}", user_key_pair.public_key);

    let mut user_cookie = Cookie::new("user_identity", user_key_pair.public_key.to_owned());
    // cookie.set_domain("hot-or-not-web-leptos-ssr.fly.dev");
    // cookie.set_expires(expiration);
    user_cookie.set_http_only(true);
    jar = jar.add(user_cookie);

    let mut exp_cookie = Cookie::new("expiration", expiration.to_string());
    exp_cookie.set_http_only(true);
    jar = jar.add(exp_cookie);

    (jar, Json(session_response))
}

// pub fn authenticate(
//     identity_keeper: State<Arc<RwLock<IdentityKeeper>>>,
//     user_oauth_id: String,
//     user_identity: String,
// ) -> Json<SessionResponse> {
// }

#[derive(Serialize)]
pub struct SessionResponse {
    user_identity: String,
    delegation_identity: agent_js::DelegationIdentity,
}

#[derive(FromRef, Clone)]
pub struct IdentityKeeper {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<RouteListing>,
    pub oauth_map: Arc<RwLock<HashMap<String, generate::KeyPair>>>,
    pub key: Key,
}

// impl FromRef<IdentityKeeper> for Key {
//     fn from_ref(state: &IdentityKeeper) -> Self {
//         state.key.clone()
//     }
// }
