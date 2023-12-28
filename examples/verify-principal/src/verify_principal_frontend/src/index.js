import { createActor } from "../../declarations/verify_principal_backend";
import { DelegationIdentity, DelegationChain } from "@dfinity/identity";
import { Secp256k1KeyIdentity } from "@dfinity/identity-secp256k1";

/*
document.querySelector("form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");

  const name = document.getElementById("name").value.toString();

  button.setAttribute("disabled", true);

  // Interact with foo actor, calling the get_principal_id method
  const greeting = await verify_principal_backend.get_principal_id();

  button.removeAttribute("disabled");

  document.getElementById("greeting").innerText = greeting;

  return false;
});
*/

$(document).ready(function(){
  $("form#get_delegated_id").click(async (e) => {
    e.preventDefault();
    // send request to axum
    $.ajax({
      method: "GET",
      url: "http://localhost:3000/auth",
      dataType: "json",
      success: function( response ) {
        // populate sections
        console.log(JSON.stringify(response));
        $("section#principal_id").text(response[0]);
        const signed_delegation = response[1];
        $("section#delegation").text(signed_delegation.delegation);
        $("section#signature").text(signed_delegation.signature);
        const delegationChain = DelegationChain.fromJSON(signed_delegation);
        const clientIdentity = Secp256k1KeyIdentity.generate();
        const delegatedIdentity = DelegationIdentity.fromDelegation(clientIdentity, delegationChain);

        const agent = new HttpAgent({
          host: IC_HOST,
          fetch,
          delegatedIdentity,
        });
        const canister_id = "bkyz2-fmaaa-aaaaa-qaaaq-cai";
        const actor = createActor(canister_id, {agent});
        const principal_id = actor.get_principal_id();
        console.log("principal_id: " + principal_id);
      },
      error: function( errorMsg) {
        console.log(errorMsg);
        $("section#axum_error_msg").text(errorMsg);
      }
    });
  });
  $("form#call_canister").click(async (e) => {
    e.preventDefault();
    // send call to canister
    // set delegated identity
    console.log("calling canister");
    const principal_id = await verify_principal_backend.get_principal_id();
    console.log("value: " + principal_id);
    // populate sections
    $("section#principal_id").text(principal_id);
  });
});
