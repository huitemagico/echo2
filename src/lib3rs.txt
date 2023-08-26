#![no_std]
//use soroban_sdk::{contract, contractimpl, symbol_short, log, Env, Symbol};
//use soroban_sdk::{contract,  contractimpl, log, symbol_short, Env, Symbol};
//add vec y Vec from hello
use soroban_sdk::{contract,  contractimpl, log, symbol_short, vec, Env, Symbol, Vec};
//use soroban_sdk::String;
use soroban_sdk::{String};


const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct Echo2Contract;
//The follwing extern is for trying to "print" things to caller.
//std::println only works in test: 
//extern crate std;
#[contractimpl]
impl Echo2Contract {
    // echo2 based on examples/events
    // version ALFA. write and read COUNTER. Not display anything.
    // Urgent to fix:
    // a. how display (log or println)
    // Step 2: accept an argument
    //use log::{info, warn};
    //pub fn echo2(env: Env) -> u32 {
   //agrego to parametro:
   //cambio el return a String
   //pub fn echo2(env: Env, to: Symbol) -> u32 {
   pub fn echo2(env: Env, to: Symbol) -> String {
        /*
         *https://www.programiz.com/rust/function
         * */
       //adding from hello:
       vec![&env, symbol_short!("Hello"), to];
        // Get the current count.
//        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
        log!(&env, "count: {}", count);
//std::println!("lib.rs ------count :{} ",count);
        // Increment the count.
        count += 1;

        // Save the count.
        //env.storage().instance().set(&COUNTER, &count);
        env.storage().persistent().set(&COUNTER, &count);

        // Publish an event about the increment occuring.
        // The event has two topics:
        //   - The "COUNTER" symbol.
        //   - The "increment" symbol.
        // The event data is the count.
        env.events()
            .publish((COUNTER, symbol_short!("echo2")), count);

        // Return the count to the caller.
//lo sig solo anda en test std::println!("lib.rs ------count :{} ",count);
/*
 *


let env = Env::default();
let msg = "a message";
let s = String::from_slice(&env, msg);
let mut out = [0u8; 9];
s.copy_into_slice(&mut out);
assert_eq!(msg.as_bytes(), out)*/


/* Pendiente 2:aqui deberia convertir el count a un String ... pero no se como hacerlo.
 * busco en https://docs.rs/soroban-sdk/latest/soroban_sdk/struct.String.html#method.to_object pero
 * no me funciona.
 * Asi que genero un String constante :-/
 * Para probar la salida de un String.
 *
 * Pendiente 3: una vez convertido el numero a String, se debe storage el String y no el u32.
 * Alternativamente se podria storage un vector con el numero y el String.*/
        //
        //
        //
//        count
//let s: String = count.to_string();
/*let env = Env::default();*/
let msg = "a message";
let s = String::from_slice(&env, msg);
//let mut out = [0u8; 9];
//s.to_object(&mut,out);
           s

    }
}

mod test;
/*[cfg(test)]

use super::{AllocContract, AllocContractClient};
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AllocContract);
    let client = AllocContractClient::new(&env, &contract_id);
    assert_eq!(client.sum(&1), 0);
    assert_eq!(client.sum(&2), 1);
    assert_eq!(client.sum(&5), 10);

    std::println!("{}", env.logs().all().join("\n"));
}
~   */
