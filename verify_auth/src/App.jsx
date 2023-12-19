import { createActor } from "../declarations/verify_principal_backend/";
import { HttpAgent, requestIdOf, SignIdentity } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { Delegation, DelegationIdentity, DelegationChain } from "@dfinity/identity";
import { Secp256k1KeyIdentity, Secp256k1PublicKey } from "@dfinity/identity-secp256k1";
import { sha256 } from '@noble/hashes/sha256';
import Secp256k1 from 'secp256k1';
import { createSignal } from 'solid-js';
import hotOrNotLogo from './assets/hotornot2.png';
import './App.css';

// const IC_HOST = "https://ic0.app";
const IC_HOST = "http://127.0.0.1:4943";
const requestDomainSeparator = new TextEncoder().encode('\x0Aic-request');

function App() {
  const [principalId, setPrincipalId] = createSignal("");

  const [delegatedIdentity, setDelegatedIdentity] = createSignal({});
  const [delegatedPrincipalId, setDelegatedPrincipalId] = createSignal("");

  const [clientTempPemPub, setClientTempPemPub] = createSignal("");
  const [clientTempPemPriv, setclientTempPemPriv] = createSignal("");
  const [signedDelegation, setSignedDelegation] = createSignal({});
  const [signaturePubkey, setSignaturePubkey] = createSignal("");

  async function generateDelegatedIdentity(e) {
    e.preventDefault();
    setClientTempPemPub("3056301006072a8648ce3d020106052b8104000a034200044cec670bc9f7858ec112dd5a9678a8c7e1c7af4ea99187adda17301bd9feef1bad41d9e15fd30c4f24a6a0bd06c8d1eb78fec95f85581ca08b785c162fd65bac");
    setclientTempPemPriv("b4904432448d39b3f6fb22be33d8d839298eb9ac5cfcc4e32555f602b46b7ca3");
    setSignedDelegation({
      "delegation": {
          "expiration": "17a1fd01422b6e00",
          "pubkey": "3056301006072a8648ce3d020106052b8104000a034200044cec670bc9f7858ec112dd5a9678a8c7e1c7af4ea99187adda17301bd9feef1bad41d9e15fd30c4f24a6a0bd06c8d1eb78fec95f85581ca08b785c162fd65bac"
      },
      "signature": "522a853b3d021ba353bcfbb884f4c48b977edd03e274a5af1a53ea49ee3c9e4d1324414bb26b372e8bff91d41202ee2e4f707a5944585abce35a5f62ffe49cc9"
    });
    setSignaturePubkey("3056301006072a8648ce3d020106052b8104000a03420004a3841f2fcb01461bfe579a6085c633d31c002abfd9dd9bbec520652ab0b3a4f58a7b76848763f2f9e2ac4f95c946cd266171a3f55f67c3124699a661ab4cdc1c");

    // fetch request to server
    const response = await fetch(
      "http://localhost:3000/auth", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: "{ 'oauth_identity': 'a793bd79c6e1f07a7830127f' }"
      }
    );
    const result = response.json().then((data) => {
      console.log(data);
      // parse data
      setPrincipalId(data[0]);
      const _inner = Secp256k1KeyIdentity.fromKeyPair(new Uint8Array(data[1]._inner[0]), data[1]._inner[1]);
      const _delegation = {
        ...data[1]._delegation,
        delegations: [
          {
            delegation: {
              ...data[1]._delegation.delegations[0].delegation,
              expiration: BigInt(data[1]._delegation.delegations[0].delegation.expiration)
            }
          }
        ]
      };
      const delegatedIdentity = DelegationIdentity.fromDelegation(_inner, _delegation);
      console.log("delegatedIdentity: ", delegatedIdentity);

      /*
      const delegatedIdentity = {
        ...data[1],
        _inner: {
          ...data[1]._inner,
          sign: async function(challenge) {
            const hash = sha256.create();
            hash.update(new Uint8Array(challenge));
            const signature = Secp256k1.ecdsaSign(
              new Uint8Array(hash.digest()),
              new Uint8Array(this[1]),
            ).signature.buffer;
            return signature;
          }
        },
        _delegation: {
          ...data[1]._delegation,
          delegations: [
            {
              delegation: {
                ...data[1]._delegation.delegations[0].delegation,
                expiration: BigInt(data[1]._delegation.delegations[0].delegation.expiration)
              }
            }
          ]
        },
        getPrincipal: function() {
          return Principal.fromText(data[2]);
        },
        transformRequest: async function(request) {
          const { body, ...fields } = request;
          const requestId = requestIdOf(body);
          return {
            ...fields,
            body: {
              content: body,
              sender_sig: await this.sign(
                new Uint8Array([...requestDomainSeparator, ...new Uint8Array(requestId)]),
              ),
              sender_delegation: this._delegation.delegations,
              sender_pubkey: this._delegation.publicKey,
            },
          };
        },
        sign: function(blob) {
          return this._inner.sign(blob);
        }        
      }
  */
      setDelegatedIdentity(delegatedIdentity);
    });
  }

  function toHexString(bytes) {
    return new Uint8Array(bytes).reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');
  }
  
  BigInt.prototype.toJSON = function() { return this.toString() }
  
  function verifyPrincipal(e) {
    e.preventDefault();
/*
    const clientIdentity = Secp256k1KeyIdentity.fromKeyPair(Uint8Array.from(clientTempPemPub()), clientTempPemPriv());

    // generate delegated identity
    const delegations = [{
      delegation: new Delegation(
        Uint8Array.from(signedDelegation().delegation.pubkey),
        signedDelegation().delegation.expiration,
      ),
      signature: Uint8Array.from(signedDelegation().signature)
    }];
    const delegationChain = DelegationChain.fromDelegations(
      delegations,
      signaturePubkey(),
    );

    const delegatedIdentity_gen = DelegationIdentity.fromDelegation(clientIdentity, delegationChain);
    setDelegatedIdentity(delegatedIdentity_gen);
    console.log("Delegated Identity", delegatedIdentity_gen);
    console.log("DI Principal: ", delegatedIdentity_gen.getPrincipal().toText());
*/

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
        <section>Delegated Id Received from server: &nbsp; {JSON.stringify(delegatedIdentity())}
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
