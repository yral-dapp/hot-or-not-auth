fn call_canister() {
    // call canister with client_identity
    let canister_id = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();
    let agent_with_client_identity = Agent::builder()
        .with_verify_query_signatures(false)
        //.with_url("https://ic0.app")
        .with_url("http://127.0.0.1:4943")
        .with_identity(client_identity)
        .build()
        .unwrap();
    agent_with_client_identity.fetch_root_key().await.unwrap();

    let user_principal_id = match agent_with_client_identity
        .query(&canister_id, "get_principal_id")
        .with_arg(Encode!().unwrap())
        .call()
        .await
    {
        Ok(resp) => Decode!(resp.as_slice(), String).unwrap(),
        Err(error) => error.to_string(),
    };

    // call canister with delegated_identity
    let agent_with_delegated_identity = Agent::builder()
        .with_verify_query_signatures(false)
        .with_url("http://127.0.0.1:4943")
        .with_identity(delegated_identity)
        .build()
        .unwrap();
    agent_with_delegated_identity
        .fetch_root_key()
        .await
        .unwrap();
    // agent_with_read_accessdelegated_identity.get_principal()
    let delegated_result = match agent_with_delegated_identity
        .query(&canister_id, "get_principal_id")
        .with_arg(Encode!().unwrap())
        .call()
        .await
    {
        Ok(resp) => Decode!(resp.as_slice(), String).unwrap(),
        Err(error) => error.to_string(),
    };
}
