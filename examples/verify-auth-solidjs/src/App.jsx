import { createActor } from "../declarations/verify_principal_backend/";
import { HttpAgent } from "@dfinity/agent";
import { Delegation, DelegationIdentity } from "@dfinity/identity";
import { Secp256k1KeyIdentity } from "@dfinity/identity-secp256k1";
import { createSignal } from 'solid-js';
import hotOrNotLogo from './assets/hotornot2.png';
import './App.css';

// const IC_HOST = "https://ic0.app";
const IC_HOST = "http://127.0.0.1:4943";

function App() {
  const [principalId, setPrincipalId] = createSignal("");

  const [delegatedIdentity, setDelegatedIdentity] = createSignal({});
  const [delegatedPrincipalId, setDelegatedPrincipalId] = createSignal("");

  async function generateDelegatedIdentity(e) {
    e.preventDefault();

    // fetch request to server
    const oauth_identity = BigInt(1703225277199791360n);

    const response = await fetch(
      "http://localhost:3000/auth", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: "{ 'oauth_identity': "+oauth_identity+" }"
      }
    );

    const result = response.json().then((data) => {
      console.log(data);
      console.log("Type of: ", typeof data[1]._delegation.delegations[0].delegation.expiration);
      // parse data
      setPrincipalId(data[0]);
      const _inner = Secp256k1KeyIdentity.fromKeyPair(new Uint8Array(data[1]._inner[0]), data[1]._inner[1]);
      const _delegation = {
        ...data[1]._delegation,
        delegations: [
          {
            ...data[1]._delegation.delegations[0],
            delegation: new Delegation(
              new Uint8Array(data[1]._delegation.delegations[0].delegation.pubkey),
              data[1]._delegation.delegations[0].delegation.expiration
            )
          }
        ]
      };
      const delegatedIdentity = DelegationIdentity.fromDelegation(_inner, _delegation);
      console.log("delegatedIdentity: ", delegatedIdentity);

      setDelegatedIdentity(delegatedIdentity);
    });
  }

  function verifyPrincipal(e) {
    e.preventDefault();

    console.log("Updated delg Idnt: ", delegatedIdentity());
    
    const agent = new HttpAgent({
      host: IC_HOST,
      fetch: window ? fetch.bind(window) : fetch,
      identity: delegatedIdentity(),
      retryTimes: 0,
      verifyQuerySignatures: false,
    });
    agent.fetchRootKey();

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
        <section>Delegated Id Received from server: &nbsp; {delegatedIdentity()}
        </section>
      </div>
      <div class="card">
        <form action="#" id="call_canister">
          <button type="button" onClick={verifyPrincipal}>Verify Delegated Id</button>
        </form>
        <section>Delegated Principal Id: {delegatedPrincipalId()}</section>
      </div>
    </>
  )
}

export default App
