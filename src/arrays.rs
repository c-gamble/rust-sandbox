//fixed length list of same data types
//use std::mem (enables mem::size_of_val())
pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers); //logs numbers
    println!("{}, {}, {}, {}, {}", numbers[0], numbers[1], 
        numbers[2], numbers[3], numbers[4]); //logs elements of numbers

    let mut new_numbers = [1, 2, 3]; /*don't need to sepcify type or 
    length of mut array */
    new_numbers[2] = 20; //re-assigns new_numbers[2] to 20

    //get array length 
    println!("{}",numbers.len());

    //arrays are stack-allocated
    println!("array occupies {} bytes", std::mem::size_of_val(&numbers));

    //get slice of array
    let slice: &[i32] = &new_numbers[0..2]; //logs [1, 2]
    println!("slice: {:?}", slice);
}   