//vectors are resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    //reassign value
    numbers[3] = 20;

    //append to vector
    numbers.push(5);
    numbers.push(6);

    //remove from vector
    println!("{:?}", numbers.pop()); //removes and returns last element
    //print entire vector
    println!("{:?}", numbers);

    //get single value
    println!("{}", numbers[0]);

    //get vector length
    println!("length: {}", numbers.len());

    //vectors are stack-allocated
    println!("vector occupes {} bytes", std::mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[1..2]; //gives 2, 3
    println!("{:?}", slice);

    //loop through vector values
    for x in numbers.iter() {
        println!("ele: {x}");
    }

    //loop and mutate 
    for x in numbers.iter_mut() {
        *x *=2;
    }
    println!("mutated vec: {:?}", numbers)
}