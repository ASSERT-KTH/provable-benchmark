fn adder(iterations: u32) {
    let mut sum = 0u32;
    for i in 0..iterations {
        sum = sum.wrapping_add(i); 
    }
    std::hint::black_box(sum); // Stops the compiler from optimising the function
}