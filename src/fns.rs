use std::time::{SystemTime};

//void fns
fn greeting(greet: &str, time: SystemTime) {
    println!("{}, user. It's {:?}", greet, time);

}
//return fns
fn add(m: i32, n: i32) -> i32 {
    return m+n;
}
pub fn run() {

    let greet = "Hello";
    let now = SystemTime::now();

    //call function
    greeting(greet, now);

    //bind function to variable 
    let result = add(1, 2);
    println!("{result}");

    //closure
    let add_nums = |a: i32, b:i32| a+b;
    let closure_sum = add_nums(3, 3);
    println!("{closure_sum}");

    //you can also use external variables in closures
    let ext_var:i32 = 4;
    let sum_nums = |p:i32, q:i32| p+q+ext_var;
    println!("{}", sum_nums(3,3)); //logs  13

}