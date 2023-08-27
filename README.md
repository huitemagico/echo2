  README.md 
  (1) name 
  echo2 "tiny piece of code for learning and practice soroban rust sdk"
  version 1.0  
  (2) function description
	 
    This code example receive an text parameter from caller, saving this text on storage,
	and return the text and the former text received.
	
	 Example of running:
	 run number 1:
	 soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release-with-logs/echo2.wasm \
    --id 1 \
    -- \
    echo2\
    --message HelloWorld

     echo2 return :
	 ["messag05","HelloWorld"]
	       |                  |
		   |                  +-----------------------echo2 return the same text received
		   |
		   +----------------this is former text passing to echo2
		   
	run number 2:	   
	soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release-with-logs/echo2.wasm \
    --id 1 \
    -- \
    echo2\
    --message HowAreYouPeople
	
	 echo2 return :
	 ["HelloWorld","HowAreYouPeople"]
		   
	(3) So, the input for echo2 is one text
	
	(4) output
    The output is the text receiving BEFORE the last call and the text received in the last call.
	
	
	 (5) Based on and tech details
	 Loosely based  from examples/events 
	 Use Persistent kind of storage instead of Instance.
	 
	 (6) Topics I have learned with corresponding links:
	 
	 a. storage . See https:docs.rs/soroban-sdk/latest/soroban_sdk/storage/index.html
	  note: persistent storage is not recomended for production environment! 
	   see https:soroban.stellar.org/docs/fundamentals-and-concepts/persisting-data
	 b. String manipulation. Obs.: rust String is different from soroban struct String . 
	                                                                https:docs.rs/soroban-sdk/latest/soroban_sdk/struct.String.html
	 c. struct Vec from soroban .                  https:docs.rs/soroban-sdk/latest/soroban_sdk/struct.Vec.html
	 d. struct Env from Soroban sdk .           https:docs.rs/soroban-sdk/latest/soroban_sdk/struct.Env.html
	 e. doc from Soroban                              https:soroban.stellar.org/docs/getting-started/setup#install-the-soroban-cli
	 f. doc for (some) examples                   https:soroban.stellar.org/docs/basic-tutorials/events
	 g. the examples                                    https:github.com/stellar/soroban-examples
	 h. rust doc sometimes gives you hope... BUT  helas!... we must use #![no_std] !!  https:doc.rust-lang.org/std/index.html
	 i. main page for soroban Rust sdk       https:soroban.stellar.org/docs/reference/sdks/rust
	 j. some shell utils to easy operation    please see main page of github for echo2 
	 l. ubuntu Rust installation                  https:linuxhint.com/rust-programming-language-ubuntu-2204/
	 
	(7)	Steps for run the program:
	If you want to test and experiment with the code steps are the following:
	Step a. download the code from github page
	Step b. check the structure for echo2, is the standard.
	 i.e. at the main directory you must run the compile, build code, and the src files (lib.rs, test.rs) 
	 are under src directory.
	 
	 Step c. at main directory run the code for compile and test.
	 cargo test -- --nocapture

     note: because of changing answers of code, testing is omitted in the test.rs
	 If you want to run with test block, simply uncomment the lines commented.
	 
	 Step d. run build code
	 soroban contract build --profile release-with-logs
	 note: if you want no logs you have to change parameters above.
	 	 
	 Step e. run code
	 soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release-with-logs/echo2.wasm \
    --id 1 \
    -- \
    echo2\
    --message HowAreYouPeople

     Note: the interesting of echo2 consist precisely in passing several parameters, so you have to change 
	 the string 'HowAreYouPeople' for another and another. This could be done with an shell loop.
	 
	 (8) My experience programming this code.
	 I began my interest on Soroban and Rust almost three weeks ago, so on these themes I am VERY young :-D
	 I could say that about Soroban SDK, is that there is good documentation about SDK (https://soroban.stellar.org/docs/fundamentals-and-concepts/high-level-overview)
	 But in my opinion (and level of expertise), there is a lack of little examples for making more easy use the SDK.
	 Nevertheless for me, beginning the journey of Rust language PLUS the Soroban SDK has been a lovely and hard work.
	 I began with the installation of UBUNTU 23 in my computer three weeks ago and then immersing in the documentation, beginning for 
	 understanding the business of Stellar, and ending for working the Soroban SDK , was a very hard work.
	 After these weeks, the list of urls listed above represent a map for continuing the travel.
	 I hope that this piece of code and urls, could serve for making easier the task people who begin this learming.
	 
	 (9) about me.
	  For me working in this project has been very exciting! 
	 I am a retired analyst and programmer who began with punched cards programming with Assembler IBM 360 in the seventys.
	 I have worked	 in PL/1, Algol, Fortran, COBOL, C.  After working as programmer, I was Business Analyst for a lot of years.
     Fortunately, last year I had the oportunity for moving the fingers on an Java project.
     And now obviously I am converted to Soroban fan :-D 	