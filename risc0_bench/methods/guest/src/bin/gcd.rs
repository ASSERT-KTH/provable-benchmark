#![allow(warnings)]
use risc0_zkvm::guest::env;

fn main(){
   let input: i32 = env::read();
   println!("Using gcd");
   let result = gcd(1432, 546232);
   env::write(&result);
}


fn gcd(m: i32, n: i32) -> i32 {
    if m == 0 {
       n.abs()
    } else {
       gcd(n % m, m)
    }
 }