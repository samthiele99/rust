// Arrays are fixed

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    //{:?} IS DEBUG traight

    numbers[0] = 8;

    numbers.push(9);

    numbers.pop();
    println!("{:?}", numbers);

    // indexing is done with []


    println!("{}", numbers.len());

    for x in numbers.iter() {
        println!("{}",x)
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);


    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));
}