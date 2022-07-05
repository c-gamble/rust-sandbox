//structs - used to create custom data types

//traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

//tuple struct
struct TupColor(u8, u8, u8);

//struct with methods
struct Person {
    first_name: String,
    last_name: String,
    age: i32
}

impl Person {
    //construct person
    fn new_person(first: &str, last: &str, age: i32) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            age: age
        }
    }

    //get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //name to tuple
    fn name_to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {

    //instantiate Color
    let mut c = Color { 
        red: 255,
        green: 0,
        blue: 0
    };
    //change props
    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    //instantiate TupColor
    let mut new_c = TupColor(255, 0, 0);

    //change props
    new_c.0 = 200;

    println!("Tuple Color: {} {} {}", new_c.0, new_c.1, new_c.2);



    //instantiate person
    let mut my_person = Person::new_person("Cooper", "Gamble", 16);

    println!("Person: {} {}, {}", my_person.first_name, my_person.last_name, my_person.age);
    println!("Full Name: {}", my_person.full_name());
    my_person.set_last_name("Gambler");
    println!("New Name: {}", my_person.full_name());
    println!("Tuple name: {:?}", my_person.name_to_tuple());
}