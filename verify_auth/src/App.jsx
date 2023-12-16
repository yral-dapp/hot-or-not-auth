import { createActor } from "../declarations/verify_principal_backend/";
import { HttpAgent } from "@dfinity/agent";
import { DelegationIdentity, DelegationChain } from "@dfinity/identity";
import { Secp256k1KeyIdentity } from "@dfinity/identity-secp256k1";
import { createSignal } from 'solid-js';
import hotOrNotLogo from './assets/hotornot2.png';
import './App.css';

// const IC_HOST = "https://ic0.app";
const IC_HOST = "http://127.0.0.1:4943";

function App() {
  const [count, setCount] = createSignal(0);
  const [principalId, setPrincipalId] = createSignal("");
  const [signedDelegation, setSignedDelegation] = createSignal({});
  const [signaturePubkey, setSignaturePubkey] = createSignal("");
  const [clientTempPemPub, setClientTempPemPub] = createSignal([]);
  const [clientTempPemPriv, setClientTempPemPriv] = createSignal([]);
  const [delegatedIdentity, setDelegatedIdentity] = createSignal({});
  const [delegatedPrincipalId, setDelegatedPrincipalId] = createSignal("");

  async function generateDelegatedIdentity(e) {
    e.preventDefault();
    // fetch request to server
    const response = await fetch(
      "http://localhost:3000/auth", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: "{ 'oauth_identity': 'Rohit Sarpotdar' }"
      }
    );
    const result = response.json().then((data) => {
      console.log(data);
      // parse data
      setPrincipalId(data[0]);
      console.log(JSON.stringify(data[1].delegation.expiration));
      setSignedDelegation(data[1]);
      setSignaturePubkey(data[2]);
      setClientTempPemPub(data[3]);
      setClientTempPemPriv(data[4]);
    });
  }

  function verifyPrincipal(e) {
    e.preventDefault();
    const clientIdentity = Secp256k1KeyIdentity.fromKeyPair(Uint8Array.from(clientTempPemPub()), Uint8Array.from(clientTempPemPriv()));

    // generate delegated identity
    const delegations = [{
      delegation: {
        pubkey: Uint8Array.from(signedDelegation().delegation.pubkey),
        expiration: signedDelegation().delegation.expiration,
      },
      signature: Uint8Array.from(signedDelegation().signature)
    }];
    console.log(delegations[0].delegation.expiration);
    const delegationChain = DelegationChain.fromDelegations(
      delegations,
      signaturePubkey(),
    );
    const delegatedIdentity = DelegationIdentity.fromDelegation(clientIdentity, delegationChain);
    setDelegatedIdentity(delegatedIdentity);

    const agent = new HttpAgent({
      host: IC_HOST,
      fetch: window ? fetch.bind(window) : fetch,
      identity: delegatedIdentity,
      retryTimes: 0,
      verifyQuerySignatures: false,
    });
    const canister_id = "bkyz2-fmaaa-aaaaa-qaaaq-cai";
    const actor = createActor(canister_id, {agent});
    actor.get_principal_id().then((principalId) => {
      console.log("principalId: " + principalId);
      setDelegatedPrincipalId(principalId);
    }).catch((reason) => {
      console.log("Why me? : " + reason);
    });
  }

  return (
    <>
      <div>
        <a href="https://hotornot.wtf" target="_blank">
          <img src={hotOrNotLogo} class="logo" alt="Hot Or Not logo" />
        </a>
      </div>
      <h3>Verification of delegated identity</h3>
      <div class="card">
        <form action="#" id="get_delegated_id">
          <button type="button" onClick={generateDelegatedIdentity}>Generate Delegated Id</button>
        </form>
        <section>User PrincipalId: {principalId()}</section>
        <section>Signed Delegation: {JSON.stringify(signedDelegation())}</section>
        <section>Signature Pubkey: {signaturePubkey()}</section>
        <section>Client Temp Session Pem Pub: {clientTempPemPub()}</section>
        <section>Client Temp Session Pem Priv: {clientTempPemPriv()}</section>
      </div>
      <div class="card">
        <form action="#" id="call_canister">
          <section>
            <label for="name">Delegated Id Received from server: &nbsp;</label>
            <input id="name" alt="Name" type="text" disabled >{delegatedIdentity()}</input>
          </section>
          <button type="button" onClick={verifyPrincipal}>Verify Delegated Id</button>
        </form>
        <section>Delegated Principal Id: {delegatedPrincipalId()}</section>
      </div>
      <div class="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count()}
        </button>
      </div>
      <p class="read-the-docs">
      </p>
    </>
  )
}

export default App
