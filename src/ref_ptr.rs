//reference pointers -> point to a resource in memory

pub fn run() {
    //primitive array
    let arr = [1, 2, 3];
    let new_arr = arr;

    println!("{:?}", (arr, new_arr));  /* logs [1,2,3], [1,2,3] */

    //with non-primtives, assigning them to another var wil make the og lose its data
    //instead, use ptrs and references
    let vec = vec![1, 2, 3];
    let new_vec = &vec;

    println!("values: {:?}", (&vec, new_vec)); /* logs [1,2,3], [1,2,3] */
}