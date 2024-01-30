use super::{agent_js, generate};
use crate::store::cloudflare::{read_metadata, write_kv};
use axum::{extract::FromRef, http::header, response::IntoResponse};
use axum_extra::extract::cookie::{Cookie, Key, SignedCookieJar};
use chrono::{Duration, Utc};
use ic_agent::{
    identity::{DelegatedIdentity, Delegation, Secp256k1Identity, SignedDelegation},
    Identity,
};
use leptos::*;
use leptos_axum::ResponseOptions;
use leptos_router::RouteListing;
use tracing::log::info;

#[server(endpoint = "generate_session")]
pub async fn generate_session() -> Result<agent_js::SessionResponse, ServerFnError> {
    let app_state: AppState = use_context::<AppState>().unwrap();
    let mut jar =
        leptos_axum::extract_with_state::<SignedCookieJar<Key>, AppState>(&app_state).await?;

    let user_identity: Option<String> = match jar.get("user_identity") {
        Some(val) => Some(val.value().to_owned()),
        None => None,
    };

    info!("User check: {:?}", user_identity);
    // client identity
    let user_key_pair: Option<generate::KeyPair> = if user_identity.is_none() {
        None
    } else {
        read_metadata(&user_identity.unwrap(), &app_state.cloudflare_config).unwrap()
        // let read_access = app_state.oauth_map.read().await;
        // read_access.get(&user_identity.unwrap()).cloned()
    };
    let user_key_pair = match user_key_pair {
        Some(kp) => kp,
        None => {
            let new_key_pair = generate::key_pair().unwrap();
            {
                let write_access = app_state.oauth_map.write();
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
    info!("client_identity: {}", client_identity.sender().unwrap());
    info!(
        "delegated_identity: {}",
        delegated_identity.sender().unwrap()
    );
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
    let session_response = agent_js::SessionResponse {
        user_identity: user_key_pair.public_key.to_owned(),
        delegation_identity: shareable_delegated_identity,
    };

    info!("user_pubkey: {}", user_key_pair.public_key);

    let mut user_cookie = Cookie::new("user_identity", user_key_pair.public_key.to_owned());
    user_cookie.set_domain("hot-or-not-web-leptos-ssr.fly.dev");
    // user_cookie.set_expires(expiration);
    user_cookie.set_http_only(true);
    jar = jar.add(user_cookie);

    let mut exp_cookie = Cookie::new("expiration", expiration.to_string());
    exp_cookie.set_domain("hot-or-not-web-leptos-ssr.fly.dev");
    // exp_cookie.set_expires(expiration);
    exp_cookie.set_http_only(true);
    jar = jar.add(exp_cookie);

    let jar_into_response = jar.into_response();
    let response = expect_context::<ResponseOptions>();
    for header_value in jar_into_response.headers().get_all(header::SET_COOKIE) {
        response.append_header(header::SET_COOKIE, header_value.clone());
    }

    Ok(session_response)
}

// pub fn authenticate(
//     app_state: State<Arc<RwLock<AppState>>>,
//     user_oauth_id: String,
//     user_identity: String,
// ) -> Json<SessionResponse> {
// }

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<RouteListing>,
    // pub oauth_map: Arc<RwLock<HashMap<String, generate::KeyPair>>>,
    pub key: Key,
    pub oauth2_client: oauth2::basic::BasicClient,
    pub reqwest_client: reqwest::Client,
    pub auth_cookie_domain: String,
    pub cloudflare_config: cloudflare_api::connect::ApiClientConfig,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}
