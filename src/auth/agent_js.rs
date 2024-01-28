use serde::{Deserialize, Serialize};

/// Structures from @dfinity/agent-js package

#[derive(Debug, Serialize)]
struct PrincipalId {
    _arr: String,
    #[serde(rename = "_isPrincipal")]
    _is_principal: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DelegationIdentity {
    pub _inner: Vec<Vec<u8>>,
    pub _delegation: DelegationChain,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DelegationChain {
    pub delegations: Vec<SignedDelegation>,
    #[serde(rename = "publicKey")]
    pub public_key: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignedDelegation {
    pub delegation: Delegation,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Delegation {
    pub pubkey: Vec<u8>,
    pub expiration: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SessionResponse {
    pub user_identity: String,
    pub delegation_identity: DelegationIdentity,
}
