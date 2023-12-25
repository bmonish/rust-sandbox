pub fn run() {
    println!("***** Print Module *****");
    // Print to Console
    println!("Hello from the Print.rs file!");

    // Basic Formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "John", "New York");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Chess"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);

    println!("**********");
    println!();
}
