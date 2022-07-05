/*
primitive types:

integers -> u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (unsigned (u) means no negatives)
floats -> f32, f64
booalean -> bool
characters -> char
tuples
arrays (statically sized)
vectors (dynamically sized)

rust is statically typed, so it must know types of all variables at compile time, but
it can usually infer the type 
*/
pub fn run() {

    //defaults to i32
    let x = 1;

    //defaults to f64
    let y = 2.5;

    //add explicit type
    let _explicit: i64 = 4545454545;

    //find max size of type
    println!("max i32: {}", std::i32::MAX);


    //booleans
    let is_active: bool = true;

    //get bool from expression
    let is_greater = 10>5;
    println!("{:?}", (x, y, _explicit, is_active, is_greater));

    //character
    let ch = '\u{1F600}';
    let ch1 = 'h';
    println!("{:?}", (ch, ch1));

}