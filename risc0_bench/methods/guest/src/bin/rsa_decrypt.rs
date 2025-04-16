#![allow(warnings)]
use risc0_zkvm::guest::env;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rand::rngs::ThreadRng;
fn main() {
    let mut rng = rand::thread_rng();
    println!("using rsa_decrypt");
    let (priv_key, pub_key, enc_data): (RsaPrivateKey, RsaPublicKey, Vec<u8>) = env::read();

            // Decrypt
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    env::commit(&dec_data);

}