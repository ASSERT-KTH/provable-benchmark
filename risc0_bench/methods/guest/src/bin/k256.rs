#![allow(warnings)]
use risc0_zkvm::guest::env;
use k256::{
    ecdsa::{SigningKey, Signature, signature::Signer},
    SecretKey,
};
use rand_core::OsRng; // requires 'getrandom' feature

fn main(){
    println!("using k256");
    // Signing
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let message = b"ECDSA proves knowledge of a secret number in the context of a single message";

    // Note: The signature type must be annotated or otherwise inferable as
    // `Signer` has many impls of the `Signer` trait (for both regular and
    // recoverable signature types).
    let signature: Signature = signing_key.sign(message);

    // Verification
    use k256::{EncodedPoint, ecdsa::{VerifyingKey, signature::Verifier}};

    let verifying_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
    assert!(verifying_key.verify(message, &signature).is_ok());
}
