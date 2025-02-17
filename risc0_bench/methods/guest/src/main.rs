use risc0_zkvm::guest::env;
use nalgebra::Matrix2;

#[allow(dead_code)]
#[allow(unused_imports)]

fn main() {

    // read the input
    let input: u32 = env::read();

    let result = adder(input);
    // write public output to the journal
    env::commit(&result);
}


fn _fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let sum = (a + b) % 7919; // Mod to avoid overflow
        a = b;
        b = sum;
    }
    b
}


fn _fibonacci2(n: u32) -> u64 {
    Matrix2::new(1, 1, 1, 0).pow(n - 1)[(0, 0)]
}

fn adder(iterations: u32) {
    let mut sum = 0u32;
    for i in 0..iterations {
        sum = sum.wrapping_add(i); 
    }
    std::hint::black_box(sum); // Stops the compiler from optimising the function
}
