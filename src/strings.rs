//primitive str = immutable, fixed-length string somewhere in memory
//string = growable, heap-allocated data structure (use when modification/ownership
// is necessary)

pub fn run() {
    //primitive string
    //let hello = "hello";

    //dynamic string
    let mut dynam = String::from("hello ");

    //get length (works for both primitive and dynamic)
    //let length = hello.len();
    //let _length = dynam.len();

    //push char
    dynam.push('W');

    //push string
    dynam.push_str("orld!");

  //println!("{:?}", (hello, dynam, length, _length)); logs hello, hello World!, 5, length of dynam 

    //check capacity
    println!("capacity: {}", dynam.capacity());
    //check if empty
    println!("Is empty? {}", dynam.is_empty()); //logs false

    //check if contains
    println!("Contains World? {}", dynam.contains("World")); //logs true
    
    //replace
    println!("Replace: {}", dynam.replace("World", "There"));

    //loop through string by whitespace
    for word in dynam.split_whitespace() {
        println!("{word}");
    }

    //create string with designated capacity
    let mut capped_string = String::with_capacity(10);
    capped_string.push('a');
    capped_string.push('b');
    println!("{capped_string}"); //logs ab

    //assertions
    assert_eq!(2, capped_string.len()); //nothing logs because it passed
    assert_eq!(3, capped_string.len()); //logs error due to inequality
    

}