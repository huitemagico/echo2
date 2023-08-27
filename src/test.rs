#![cfg(test)]
use soroban_sdk::{String};

use super::*;


#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Echo2Contract);
    let client = Echo2ContractClient::new(&env, &contract_id);

    

    // Step 1: Send first message
    let first_message=String::from_slice(&env, "One");
    let expected_original_message=String::from_slice(&env, "NoOldMessage");
    let echo_response=client.echo2(&first_message);
    
    let expected_echo_response=vec![&env, 
        expected_original_message,
        first_message.clone()
        ];
    assert_eq!(echo_response, expected_echo_response);

    // Step 2: Send second message:
    let second_message=String::from_slice(&env, "Two");
    let echo_response=client.echo2(&second_message);

    let expected_echo_response=vec![&env, 
            first_message,
            second_message.clone()
        ];
    assert_eq!(echo_response, expected_echo_response);

    // Step 3: Send second message:
    let third_message=String::from_slice(&env, "Third");
    let echo_response=client.echo2(&third_message);

    let expected_echo_response=vec![&env, 
            second_message,
            third_message
        ];
    assert_eq!(echo_response, expected_echo_response);



}