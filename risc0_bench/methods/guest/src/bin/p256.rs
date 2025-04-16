#![allow(warnings)]
use risc0_zkvm::guest::env;
use p256::{
    ecdsa::{SigningKey, Signature, signature::Signer},
};
//use rand_core::OsRng; // requires 'getrandom' feature
use rand::rngs::OsRng;

fn main() {
    println!("using p256");
    p256();
}


fn p256() {
    // Signing
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let message = b"Hello, World!";
    let signature: Signature = signing_key.sign(message);

    // Verification
    use p256::ecdsa::{VerifyingKey, signature::Verifier};

    let verifying_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
    assert!(verifying_key.verify(message, &signature).is_ok());
}