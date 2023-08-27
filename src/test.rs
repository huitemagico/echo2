#![cfg(test)]

use super::*;


#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Echo2Contract);
    let client = Echo2ContractClient::new(&env, &contract_id);


client.echo2(&symbol_short!("Dev"));

}

