// Arrays are fixed

use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1,2,3,4,5];

    //{:?} IS DEBUG traight
    println!("{:?}", numbers);

    // indexing is done with []


    println!("{}", numbers.len());


    println!("Array occupies {} bytes", mem::size_of_val(&numbers))
}