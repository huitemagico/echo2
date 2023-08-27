#![no_std]
use soroban_sdk::{contract,  contractimpl, symbol_short, vec, Env, Symbol, Vec,String};
mod test;

const OLD_MSG: Symbol = symbol_short!("OLD_MSG");



#[contract]
pub struct Echo2Contract;

#[contractimpl]
impl Echo2Contract {
    pub fn echo2(env: Env, message: String) -> Vec<String> {

        // Get the old message
        let old_message=env.storage()
                            .persistent()
                            .get(&OLD_MSG)
                            .unwrap_or(String::from_slice(&env, "NoOldMessage")); 
        
        // Save new message in OLD_MSG storage
        env.storage().persistent().set(&OLD_MSG, &message);

        vec![&env, old_message,message]    
    }
    
   
    
}