# Anonymous delegation using Canister Signature

```mermaid
---
title: Generate anonymous delegation using canister signature
---
sequenceDiagram
    actor client as Client Device
    participant ssr as SSR Backend
    participant auth as Auth Service
    participant signer as Canister Signer
    participant kv as Cloudflare KV Store
    participant canister as User Canister
    client->>ssr: First time visit
    ssr-->>client: Embeds anonymous page in iframe
    client->>auth: Loads anonymous page in iframe
    Note over auth: Generates random seed<br/> as SessionKey/PubKey
    Note over auth: SessionKey is private with auth.<br/> Generated UserKey is signed and<br/> used as refresh token.
    auth->>signer: Calls prepare_delegation
    Note over signer: Using random_seed<br/> generates UserKey
    signer-->>auth: Sends user_key, expiration
    auth-)kv: Saves random_seed, user_key
    auth->>signer: Calls GetDelegation
    signer-->>auth: Returns SignedDelegation
    auth-->>client: Returns SignedDelegation via PostMessage<br/> Stores user_key signed by auth, in cookies<br/> as a refresh token
    client->>canister: Using SignedDelegation request for resources
    Note over canister: Using SignedDelegation, verify signature
    canister-->>client: Returns resources
```
