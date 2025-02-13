use risc0_zkvm::guest::env;
use std::hint::black_box; //Part of rust std lib that stops the compiler from optimizing away certain things,
                          //potentially ruining benchmarks

fn main() {

    // read the input
    let input: u32 = env::read();

    let result = black_box(fibonacci(black_box(input)));
    // write public output to the journal
    env::commit(&result);
}


fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let sum = (a + b) % 7919; // Mod to avoid overflow
        a = b;
        b = sum;
    }
    b
}


