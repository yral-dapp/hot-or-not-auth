# Client Canister authentication Flow

## No OAuth verification flow
### Generated identity & session identity on the server for new user

```mermaid
---
title: Generated identity & session identity on the server for new user
---
sequenceDiagram
    actor client as Client Device
    participant ssr as SSR Backend
    participant auth as Auth Service
    participant canister as Canister
    participant kv as Cloudflare KV Store
    client->>ssr: Visits website 1st time
    ssr->>auth: Requests for Session KeyPair
    Note over auth: Creates Private KeyPair <br/> & Session KeyPair for a user <br/> using random seed
    Note over auth: Session KeyPair is <br/> valid for 30 minutes
    auth->>kv: Store User's private & session KeyPair
    Note over kv: {pubkey: User's Pubkey, <br/> private_key: User's Private Key, <br/> session_identity: Session KeyPair }
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
```

### Client flow when session expires, visits after 30 minutes

```mermaid
---
title: Generates new session keypair based on previous session keypair
---
sequenceDiagram
    actor client as Client Device
    participant ssr as SSR Backend
    participant auth as Auth Service
    participant canister as Canister
    participant kv as Cloudflare KV Store
    client->>canister: Fetch video resources
    canister-->>client: Session Timeout
    client->>ssr: Ask for new session KeyPair <br/> by providing <br/> expired Session Keypair & Signature
    critical Check if its valid Previous Session KeyPair
        ssr->>auth: Sends Previous Session KeyPair & Signature
        option Valid Pre-Session
            Note over auth: Generates new Session KeyPair
            auth->>kv: Updates Session KeyPair
            auth-->>ssr: Sends New Session KeyPair
            ssr-->>client: Receives new Session KeyPair
            client->>canister: Fetches video resources
            canister-->>client: Receives video resources
        option Invalid Pre-Session
            Note over auth: Continues flow as per <br/> 'First time website visit' <br/> Generates new Private identity <br/> & Session KeyPair
    end
```


## OAuth Login when user wants to claim tokens
```mermaid
---
title: OAuth Login when user wants to claim tokens
---
sequenceDiagram
    actor client as Client Device
    participant ssr as SSR Backend
    participant auth as Auth Service
    participant ext_auth as External OAuth Service
    participant canister as Canister
    participant kv as Cloudflare KV Store
    client->>ssr: Client clicks on claim tokens button
    ssr-->>client: Returns Login Page with oAuth providers
    client->>ext_auth: Chooses provider & redirects / popup
    ext_auth-->>client: Provider passes OAuth Id & Token to SSR backend
    client->>ssr: Psses OAuth Id, Token, <br/> Session KeyPair & Signature
    ssr->>auth: Passes OAuth Id, Token, <br/> Session KeyPair & Signature
    critical Is Session valid?
        auth-->>auth: Chechks if Session KeyPair <br/> & Signature is valid
        option Valid
            auth->>kv: Adds OAuth Id & Token <br/> for User's Private KeyPair
            auth-->>ssr: Verification confirmed
            Note over ssr: Prepares for tokens claim
            ssr-->>client: Sends updated page
        option Invalid
            Note over auth: Generates new User's Private KeyPair <br/> & new Session KeyPair
            auth->>kv: Stores all data for new user
            Note over kv: {pubkey: User's pubkey, <br/> private_key: User's private key, <br/> session_identity: Session KeyPair, <br/> & OAuth Id, Token}
            auth-->>ssr: Returns Session KeyPiar & Delegated Signature
            ssr->>canister: Fetches resources using delegated identity
            ssr-->>client: Returns statig page
    end
    Note over client: Client continues using app & can claim tokens
```

### Renew Session KeyPair before expiry on client

```mermaid
---
title: Renew session KeyPair when expiration is less than 5 minutes away
---
sequenceDiagram
    actor client as Client Device
    participant ssr as SSR Backend
    participant auth as Auth Service
    participant canister as Canister
    participant kv as Cloudflare KV Store
    critical Check if Delegated Identity expiration has less than 5 minutes
    option Less than 5 Mins
        client->>auth: Ask for Session KeyPair renewal
        auth-->>client: Validates signature <br/> & renews Session KeyPair
        auth-)kv: Updates Session KeyPair for the user
    option Greater than 5 Mins
        Note over client: No action <br/> required
    end
    client->>canister: Continue requesting new resources
```
