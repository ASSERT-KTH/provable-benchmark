use risc0_zkvm::guest::env;
use password_hash::{PasswordHash, PasswordVerifier};
use argon2::Argon2; 

fn main() {
}
pub fn passhash() {
    // Can be: `$argon2`, `$pbkdf2`, or `$scrypt`
    let hash_string = "$argon2i$v=19$m=65536,t=1,p=1$c29tZXNhbHQAAAAAAAAAAA$+r0d29hqEB0yasKr55ZgICsQGSkl0v0kgwhd+U3wyRo";
    let input_password = "password";

    let password_hash = PasswordHash::new(&hash_string).expect("invalid password hash");

    // Trait objects for algorithms to support
    let algs: &[&dyn PasswordVerifier] = &[&Argon2::default()];

    password_hash.verify_password(algs, input_password).expect("invalid password"); 

}