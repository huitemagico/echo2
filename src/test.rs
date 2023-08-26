#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Events, vec, Env, IntoVal};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Echo2Contract);
    let client = Echo2ContractClient::new(&env, &contract_id);
//let words = client.hello(&symbol_short!("Dev"));
//si echo2 devuelve String... cambio 1 por "1"
/*let env = Env::default();
let msg = "a message";
let s = String::from_slice(&env, msg);
let mut out = [0u8; 9];
s.copy_into_slice(&mut out);
assert_eq!(msg.as_bytes(), out)*/
/*
 *
 *#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Logs, Address, BytesN, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();

    let id_bytes = BytesN::from_array(&env, &[8; 32]);
    let contract_id = env.register_contract(&Address::from_contract_id(&id_bytes), Contract);
    let client = ContractClient::new(&env, &contract_id);

    client.hello(&symbol_short!("Dev"));

    let logs = env.logs().all();
    assert_eq!(logs, std::vec!["[Diagnostic Event] contract:0808080808080808080808080808080808080808080808080808080808080808, topics:[log], data:[\"Hello {}\", Dev]"]);
    std::println!("{}", logs.join("\n"));
}

 *
 *
 * */
client.echo2(&symbol_short!("Dev"));
}

/*
    assert_eq!(client.echo2(&symbol_short!("Dev")), "1");
    assert_eq!(client.echo2(&symbol_short!("Dev")), "2");
    assert_eq!(client.echo2(&symbol_short!("Dev")), "3");

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("echo2")).into_val(&env),
                1u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("echo2")).into_val(&env),
                2u32.into_val(&env)
            ),
            (
                contract_id,
                (symbol_short!("COUNTER"), symbol_short!("echo2")).into_val(&env),
                3u32.into_val(&env)
            ),
        ]
    );
}
*/

/*
 * el etst del hello con dos parametros
 * use crate::{Contract, ContractClient};
use soroban_sdk::{symbol_short, vec, Env};

#[test]
fn hello() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol_short!("Dev"));
    assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    );
}
~ */
