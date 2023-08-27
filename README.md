  README.md 
  (1) name 
  echo2 "tiny piece of code for learming and practice soroban rust sdk"
  version 1.0  
  (2) function description
	 write and read an counter.
	 
    This code example receive an parameter from caller, and return a counter of succesive runs
	and a vector of numbers.
	(3) input
	one word
	(4) output
    The 	return values consist of n-tupla (Symbol, String, and array of u32 values. 
	 "pseudo random pending."
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
	a. download the code from github page
	b. be sure set the structure for echo2 as follows:
	 (main directory)
	 |
	 |
	 +-------src
	 |           +-------------------lib.rs test.rs 
	 |
	 +-------release 
	 etc
	 c. at main directory run the shell for compile and test
	 d. run build shell
	 e. run execute shell
	 
	 (8) My experience of this journey
	 My experience programming this code. About Soroban SDK, is that there is good documentation about SDK (https://soroban.stellar.org/docs/fundamentals-and-concepts/high-level-overview)
	 Nevertheless for me, beginning the journey of Rust language PLUS the Soroban SDK has been lovely and hard work.
	 I began with the installation of UBUNTU 23 in my computer three weeks ago and then immersing in the several documentation, was a little confusing.
	 After these weeks, the list of urls listed above represent a kind of treasure, and a map for continuing the travel.
	 I hope that this piece of code and urls, serve for facilitate the task people who begin this learming.
	 
	 (9) about me.
	 I am a retired analyst and programmer who began with punched cards programming with Assembler IBM 360 in the seventys, I have worked
	 in PL/1, Algol, Fortran, COBOL, C, Java.  For me working in this project has been very exciting! 