

pub fn run() {

    //infinite loop
    let mut count = 0;
    loop {
        count += 1;
        println!("{count}");

        if count == 20 {
            break;
        }
    }
    //for range loop
    for x in 0..100 {
        println!("{x}"); //logs numbers 1-100
        continue; 
    }


    //while loops
    let mut i = 0;
    while i <= 100 {
        if i % 5 == 0 {
            println!("buzz");
        }
        else if i % 3 == 0 {
            println!("fizz");
        }
        else if i % 15 == 0 {
            println!("fizzbuzz");
        } else {println!("{i}")}
        i += 1;
    }
}