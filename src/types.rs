/*
Primitive Types:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays - Fixed Length
*/

// Statically Typed language, but the compiler can usually infer what type we want to use based on the value

pub fn run() {
    println!("***** Types Module *****");

    // Default is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 12312313213123;

    // Find max side
    println!("Max i32 is {}", std::i32::MAX);
    println!("Max i32 is {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    // Character
    let c1 = 'a';
    // Supports Unicode as well
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, c1, face));

    println!("**********");
    println!();
}
