// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 11];

    // Print whole vector
    println!("{:?}", numbers);

    // Get single value
    println!("First Index: {}", numbers[0]);

    // Re-assign value
    numbers[2] = 20;

    // Adding elements to vector
    numbers.push(7);
    numbers.push(8);
    numbers.push(9);

    // Pop last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies: {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop Vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Vector after multiplying by 2: {:?}", numbers);
}
