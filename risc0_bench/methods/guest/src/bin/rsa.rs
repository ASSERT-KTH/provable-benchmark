#![allow(warnings)]
use risc0_zkvm::guest::env;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rand::rngs::ThreadRng;
fn main() {
    let mut rng = rand::thread_rng();
    println!("using rsa");
    let (priv_key, pub_key): (RsaPrivateKey, RsaPublicKey) = env::read();
        // Encrypt
    let data = b"hello world";
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);
    
        // Decrypt
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]); 
}
