#![no_std]
use soroban_sdk::{contract,  contractimpl, symbol_short, vec, Env, Symbol, Vec,String};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct Echo2Contract;
#[contractimpl]
impl Echo2Contract {
    // echo2 
	//
	//
    // version 1.0  write and read COUNTER. 
	//Display n-tupla of returning values (Symbol, String, and array of u32 values. pseudo random pending.
	// Based loosely from examples/events changing Instance to Persistent kind of storage
	// Topics I have learned and that I give the links to the community
	// 1. storage . See https://docs.rs/soroban-sdk/latest/soroban_sdk/storage/index.html
	//  note: persistent is not recomended for production environment! https://soroban.stellar.org/docs/fundamentals-and-concepts/persisting-data
	// 2. String manipulation. Obs.: rust String is different from soroban struct String . 
	//                                                                https://docs.rs/soroban-sdk/latest/soroban_sdk/struct.String.html
	// 3. struct Vec from soroban .                  https://docs.rs/soroban-sdk/latest/soroban_sdk/struct.Vec.html
	// 4. struct Env from Soroban sdk .           https://docs.rs/soroban-sdk/latest/soroban_sdk/struct.Env.html
	// 5. doc from Soroban                              https://soroban.stellar.org/docs/getting-started/setup#install-the-soroban-cli
	// 6. doc for (some) examples                   https://soroban.stellar.org/docs/basic-tutorials/events
	// 7. THE examples                                   https://github.com/stellar/soroban-examples
	// 8. helas! rust doc sometimes gives you hope... BUT we must use #![no_std] !!  https://doc.rust-lang.org/std/index.html
	// 9. main page for soroban Rust sdk       https://soroban.stellar.org/docs/reference/sdks/rust
	// 10. some shell utils to easy operation    please see main page of github for echo2 
	// 11. ubuntu Rust installation                  https://linuxhint.com/rust-programming-language-ubuntu-2204/
	
    
	  pub fn echo2(env: Env, to: Symbol) -> (Symbol, String   ,Vec<u32>) {
	  
	  
	  let mut a: [u32; 6]  = [1, 2, 3, 4, 5, 6];

    
	a[2]=100;
	
 
	
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        count += 1;

        // Save the count.

        env.storage().persistent().set(&COUNTER, &count);

        /*
        env.events()
            .publish((COUNTER, symbol_short!("echo2")), count);
*/

		let _msg = "a message";
let smsg = String::from_slice(&env, _msg);


if count > 51 {

	a[1]=count;
	a[2]=five();
	a[3]=five();
	a[4]=five();
	a[5]=five();
	let sd1:u32 =123;

	a[5]=ra(sd1) ;
 }
		   return (to, smsg,vec![&env,a[5],a[2],a[3],a[4],a[1]])
		   

    }
}

mod test;
fn five() -> u32 {
    5
}


fn ra(_sd: u32 ) ->u32 {
   //pending
   let r:u32=1;
   
   
   r
   
   
   
}