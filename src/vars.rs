// Variables are immutable by default
// Rust is a block-scoped language
// Variables hold primitive data or references to data

pub fn run() {
    println!("***** Variable Module *****");
    let name = "John";
    let mut age = 23;
    println!("My name is {} and I am {}", name, age);

    age = 24;
    println!("My name is {} and I am {}", name, age);

    // Define constant and type is mandatory
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("John", 23);
    println!("{} is {}", my_name, my_age);

    println!("**********");
    println!();
}
