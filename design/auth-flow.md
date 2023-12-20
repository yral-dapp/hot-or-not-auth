# Client Authentication Flow


```mermaid
---
title: Client Authentication flow using external OAuth provider
---
sequenceDiagram
    actor client as Client Device
    participant ssr as SSR Backend
    participant auth as Auth Service
    participant ext_auth as External OAuth Provider
    participant canister as Canister
    participant kv as Cloudflare KV Store
    client->>ssr: Visits website 1st time
    ssr-->>client: Returns Login Page with oAuth providers
    client->>ext_auth: Chooses provider & redirects / popup
    ext_auth-->>client: Provider passes OAuth Id & Token to SSR backend
    client->>ssr: Psses OAuth Id & Token
    ssr->>auth: Passes OAuth Id & Token
    Note over auth: Creates Private KeyPair <br/> & Pession KeyPair for a user <br/> using OAuth Id as seed
    Note over auth: Session KeyPair is <br/> valid for 30 minutes
    auth->>kv: Store User's private & session KeyPair
    Note over kv: {key_id: OAuth Id, <br/> main_identity: Private KeyPair, <br/> session_identity: Session KeyPair}
    Note over auth: Generates DelegatedIdentity <br/> signed by Private KeyPair
    auth-->>ssr: Returns Delegated Identity
    Note over ssr: Builds Secp256k1Identity <br/> & DelegatedIdentity
    ssr->>canister: Requests resources using DelegatedIdentity
    canister-->>ssr: Resources returned
    Note over ssr: Build static page with resources loaded
    Note over ssr: Adds DelegatedIdentity <br/> serialized for front-end <br/> to use for subsequent calls
    ssr-->>client: Returns static page with DelegatedIdentity
    Note over client: Client has ready to consume videos
    Note over client: Client builds Secp256k1KeyIdentity <br/> & DelegationIdentity <br/> & keeps ready for <br/> future canister calls
    client->>canister: When needed calls canister directly for fetching resources
    canister-->>client: Provides resources
