#![no_std]
// version 1.1 reset
use soroban_sdk::{contract,  contractimpl, symbol_short, vec, Env, Symbol, Vec,String};
const COUNTER: Symbol = symbol_short!("COUNTER");
const OLD_MSG: Symbol = symbol_short!("OLD_MSG");
mod test;

#[contract]
pub struct Echo2Contract;

#[contractimpl]
impl Echo2Contract {
    pub fn echo2(env: Env, message: String) -> (u32,u32,Vec<String>) {
	let _old_message = "nomessage";
//
//
	let resetmessage = String::from_slice(&env, "reset");

//save for work message received
	let  ln1:u32;
	ln1=message.len();
//
	let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
	count += 1;

// Get the old message
	let mut old_message=env.storage()
                            .persistent()
                            .get(&OLD_MSG)
                            .unwrap_or(String::from_slice(&env, "NoOldMessage0")); 
							
	if message ==resetmessage {
			 old_message=String::from_slice(&env, "ResetMessageStored"); 
		}
//

	let msg = "echo2 v.1.1 27/08/2023";
//
	let sout = String::from_slice(&env, msg);
			
// Save new message in OLD_MSG storage
        env.storage().persistent().set(&OLD_MSG, &message);
		env.storage().persistent().set(&COUNTER, &count);
		
     return(   ln1, count,vec![&env, sout, old_message,message]    )
    }
    

}