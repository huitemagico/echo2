#![cfg(test)]
use soroban_sdk::{String};
use super::*;


#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Echo2Contract);
    let client = Echo2ContractClient::new(&env, &contract_id);


    // Step 1: Send first message
    let first_message=String::from_slice(&env, "reset");
	//[5,170,["echo2 v.1.1 27/08/2023","ResetMessageStored","reset"]]

    //
    let echo_response_tupla=client.echo2(&first_message);
	let expected_echo_response=5;
		
		
   assert_eq!(echo_response_tupla.0, expected_echo_response);
  


}