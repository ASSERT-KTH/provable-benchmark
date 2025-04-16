#![allow(warnings)]
use risc0_zkvm::guest::env;
use tiny_keccak::{Keccak, Hasher};


fn main() {
    keccak();
}

fn keccak(){
    let mut hasher = Keccak::v256();    
    let input_a = b"hello world";
    let input_b = b"!";
    let mut output = [0u8; 32];
    hasher.update(input_a);
    hasher.update(input_b);
    hasher.finalize(&mut output);
}
