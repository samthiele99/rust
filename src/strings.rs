// Primitive str = immutable
// String = growable 

pub fn run() {
    let mut hello = String::from("Hello");


    println!("Length: {}", hello.len());

    //push char
    hello.push('W');

    //push string

    hello.push_str("world!");

    println!("Capacity: {}", hello.capacity());

    println!("Is empty: {}", hello.is_empty());

    println!("Contains World: {}", hello.contains("World"));

    println!("Contains World: {}", hello.replace("Hello","You"));

    for char in hello.chars() {
        println!("{}",char);
    }


    let mut s = String::with_capacity(10);

    s.push('a');

    s.push('b');

    assert_eq!(2,s.len());

}