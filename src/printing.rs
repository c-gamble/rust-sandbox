pub fn run() {
    //basic print
    println!("this is the: {} file", "test_fn");
    
    //basic formatting
    println!("in {} year, i will be {} years old", 1, 17);

    //positional arguments
    println!("{0} is from {1} and {0} likes to code", "cooper", "minnesota");

    //named arguments
    println!("{name} likes to {activity}", name = "Cooper", activity = "play piano");

    //placeholder traits
    println!("Binary: {:b}, Hexadecimal: {:x}, Octal: {:o}", 10, 10, 10);

    //placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //basic math
    println!("10+10 = {}", 10+10);
}


