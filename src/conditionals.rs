pub fn run() {
    let age = 22;
    let check_id: bool = true;

    if age  >= 21 || check_id == true { //or operator is ||
        println!("you can drink");
    }
    else if age >=21 && check_id == false { //and operator is &&
        println!("sherly temple?");
    }
    else {
        println! ("you're too young buddy");
    }

    //shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("{is_of_age}");
}
