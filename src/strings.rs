// Primitive str = Immutable fixed-length string stored somewhere in memory
// String = Growable, heap-allocated data structure.

pub fn run() {
    // Primitive
    let hello = "Hello";

    // Growable String
    let mut heap_hello = String::from("Hello from Heap! ");

    // Get Length
    println!("Length: {}", hello.len());

    // Adding Characters
    heap_hello.push('W');

    // Push Str
    heap_hello.push_str("elcome");

    // Capacity in bytes
    println!("Capacity: {}", heap_hello.capacity());

    // isEmpty
    println!("Is Empty: {}", heap_hello.is_empty());

    // Contains
    println!("Contains 'World' {}", heap_hello.contains("We"));

    // Replace
    println!("Replace: {}", heap_hello.replace("Welcome", "Hi There"));

    // Loop through string by whitespace
    for word in heap_hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('1');
    s.push('2');
    s.push('3');
    s.push('4');
    s.push('5');
    s.push('6');
    s.push('7');
    s.push('8');
    s.push('9');
    s.push('j');

    // Assertion
    assert_eq!(10, s.len());
    assert_eq!(10, s.capacity());

    println!("{:?}", (hello, heap_hello, s));
}
