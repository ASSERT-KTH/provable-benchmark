use risc0_zkvm::guest::env;
use std::hint::black_box; //Part of rust std lib that stops the compiler from optimizing away certain things,
                          //potentially ruining benchmarks
use nalgebra::Matrix2;

#[allow(dead_code)]
#[allow(unused_imports)]

fn main() {

    // read the input
    let input: u32 = env::read();

    //let result = black_box(fibonacci(black_box(input)));
    //let result = fibonacci2(input);
    let result = adder(input);
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


fn fibonacci2(n: u32) -> u64 {
    Matrix2::new(1, 1, 1, 0).pow(n - 1)[(0, 0)]
}

fn adder(iterations: u32) {
    let mut sum = 0u32;
    for i in 0..iterations {
        sum = sum.wrapping_add(i); // Simple computation to prevent optimization
    }
    std::hint::black_box(sum); // Prevents the compiler from optimizing away the loop
}
