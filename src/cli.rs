use std::env;

pub fn run() {
    // First argument will be target of the executable
    let args: Vec<String> = env::args().collect();
    let name = "John";
    let status = "100%";

    // First user argument
    let command = if args.len() > 1 {
        args[1].clone()
    } else {
        "".to_string()
    };

    println!("Args: {:?}", args);
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command!")
    }
}
