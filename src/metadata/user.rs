use crate::{
    auth::{agent_js, generate::from_hex_string, identity::AppState},
    store::cloudflare::{read_metadata, write_kv},
};
use axum::{extract::State, Json};
use ic_agent::{
    identity::{DelegatedIdentity, Delegation, Secp256k1Identity, SignedDelegation},
    Identity,
};
use k256::SecretKey;
use serde::Deserialize;
use std::collections::HashMap;
use tracing::log::info;

pub async fn update_user_metadata(
    State(app_state): State<AppState>,
    Json(user_details): Json<UserDetails>,
) -> Result<String, String> {
    let delegated_identity: DelegatedIdentity =
        user_details.delegation_identity.try_into().unwrap();
    let user_principal_id = delegated_identity.sender().unwrap();
    let user_pubkey = delegated_identity.public_key().unwrap();
    let mut metadata: HashMap<&str, &str> = HashMap::with_capacity(2);
    metadata.insert("user_canister_id", &user_details.user_canister_id);
    metadata.insert("user_name", &user_details.user_name);

    let _ignore = write_kv(
        &user_principal_id.to_text(),
        "",
        metadata,
        &app_state.cloudflare_config,
    )
    .await;

    Ok("Successful".to_owned())
}

pub async fn get_user_canister(
    State(app_state): State<AppState>,
    Json(user_principal_id): Json<String>,
) -> Result<String, String> {
    info!("{}", user_principal_id);

    match read_metadata(&user_principal_id, &app_state.cloudflare_config).await {
        Some(user_metadata) => {
            let user_canister_id = match user_metadata.get("user_canister_id") {
                Some(c) => c,
                None => return Err("User details not found".to_owned()),
            };
            Ok(user_canister_id.to_owned())
        }
        None => Err("User not found".to_owned()),
    }
}

async fn verify_signature(signature: String) -> bool {
    true
}

impl TryFrom<agent_js::DelegationIdentity> for DelegatedIdentity {
    type Error = String;

    fn try_from(delegation_identity: agent_js::DelegationIdentity) -> Result<Self, Self::Error> {
        info!("DelegationIdentity into");
        let js_signed_delegation = delegation_identity._delegation.delegations;
        if js_signed_delegation.len() == 0 {
            return Err("No signed delegations found".to_owned());
        }
        let js_signed_delegation = js_signed_delegation.get(0).unwrap();
        let js_delegation = js_signed_delegation.delegation.to_owned();
        let exp1 = match from_hex_string(&js_delegation.expiration) {
            Ok(val) => val,
            Err(error) => return Err(error),
        };

        let delegation = Delegation {
            pubkey: js_delegation.pubkey,
            expiration: exp1,
            targets: None,
        };
        let signed_delegation = SignedDelegation {
            delegation,
            signature: js_signed_delegation.signature.to_owned(),
        };

        let secret_key = SecretKey::from_slice(delegation_identity._inner[1].as_slice()).unwrap();
        let client_temp_identity = Secp256k1Identity::from_private_key(secret_key);
        let delegated_identity = DelegatedIdentity::new(
            delegation_identity._delegation.public_key,
            Box::new(client_temp_identity),
            vec![signed_delegation],
        );
        Ok(delegated_identity)
    }
}

#[derive(Deserialize, Debug)]
pub struct UserDetails {
    delegation_identity: agent_js::DelegationIdentity,
    user_canister_id: String,
    user_name: String,
}
