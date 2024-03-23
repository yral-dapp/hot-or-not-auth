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
    Note over auth: Auth master key<br/> is updated periodically
    Note over signer: Auth will renew<br/> its identity with signer<br/> using canister call
    Note over auth: Using private<br/> master key/auth cert<br/> create ic-agent for canister call
    Note over auth: Using random seed<br/> creates 32 bytes random string<br/> which will be identity of user
    Note over auth: UserIdentity is public<br/> signed UserIdentity is<br/> used as refresh token.
    auth-)auth: Generate SessionKeyPair<br/> Changes on session renewal
    auth->>signer: Calls get_signed_delegation<br/> with SessionPubKey
    Note over signer: Generates SignedDelegation<br/> using canister cert
    signer-->>auth: Returns SignedDelegation
    auth-)kv: Saves UserIdentity to KV
    auth-)auth: Uses SessionKeyPair<br/> Creates DelegatedIdentity
    auth-->>client: Returns DelegatedIdentity via PostMessage<br/> Stores UserIdentity signed<br/> by auth master key, in cookies/JWT<br/> as a refresh token
    client->>canister: Using DelegatedIdentity request for resources
    Note over canister: Using DelegatedIdentity, verify signature
    canister-->>client: Returns resources
```

