Authentication service for Hot Or Not

![Target Architecture](https://github.com/go-bazzinga/hot-or-not-auth/blob/main/design/auth-flow.md)


## Current POC Architecture
1. Initiate request with oauth_id to Auth service

1.a Auth service generates (for the 1st time) keypair (kept on server-side) for user. Also generates session keypair for the client (TTL 1 hour).

Prepares Delegation Identity.

2. Sends Delegated Identity object to frontend

2.a. Frontend recreates delegated identity
    Creates Actor and sends call to canister method. 
    Internally it needs methods of DelegationIdentity & Secp256k1KeyIdentity.
    Internally it needs to sign the message before sending request to canister.


## Target Architecture
1. Initial request with oauth_id to SSR after OAuth login

1.a. SSR passes oauth_id to Auth service

Auth service generates (for the 1st time) keypair (kept on server-side) for user. Also generates session keypair for the client (TTL 1 hour).

Prepares Delegation Identity.

2. Sends Delegated Identity object to SSR.

2.a. SSR recreates Delegated Identity sends canister request to fetch resources for user. Prepares page with Delegated Identity added in it.

2.b. Send the page to user.

3. User recreates delegated identity coming from SSR, and sends request to canister.


Note: Right now in POC, SSR is ommitted and Front-end directly connecting to auth service

## How to build

```bash
cargo leptos build --release
```
