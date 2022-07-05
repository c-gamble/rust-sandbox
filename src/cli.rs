pub fn run() {

    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();

    println!("Command: {:?}", command);
    
    
    let name = "Cooper";

    let status = 100;

    if command == "hello" {
        println!("{name}");
    } else if command == "status" {
        println!("{status}");
    }

    
}