//tuples group together values of different or alike types
//max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Cooper", "Gamble", 16);
    println!("{} {} is {} years old", person.0, person.1, person.2);
}