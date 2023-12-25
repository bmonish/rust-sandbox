// Arrays - Fixed list where elements are the same data types

// Importing modules
use std::mem;

pub fn run() {
    let mut numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];

    // Print whole array
    println!("{:?}", numbers);

    // Get single value
    println!("First Index: {}", numbers[0]);

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
