#![allow(warnings)]
use risc0_zkvm::guest::env;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rand::rngs::ThreadRng;
fn main() {
    let mut rng = rand::thread_rng();
    println!("using rsa_encrypt");
    let (priv_key, pub_key): (RsaPrivateKey, RsaPublicKey) = env::read();
        // Encrypt
    let data = b"hello world";
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    env::commit(&enc_data);

}