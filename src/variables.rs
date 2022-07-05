//variables hold primitive data or references to data
//variables are immutable by default
//rust is a block-scoped language

pub fn run() {
    let name = "cooper";
    let mut age = 16; //this is how you make a mutable variable
    println!("{age}"); //returns 16
    age = 17;
    println!("my name is {name}, and I am {age} years old.");

    //const keyword
    const ID: i32 = 001;
    println!("id: {ID}");

    //assign multiple variables
    let (mname, mage) = ("member name", "member age");
    println!("{mname} is {mage}");
}