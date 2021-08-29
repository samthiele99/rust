// Integers can be unsigned eg u8 or signed eg i16, if they are unsigned they cannot be negative
// Floats 32 or 54
// Bool
// Char
// tuples
// arrays are fixed length
// Rust is statically typed, but the compiler can infer types 

pub fn run() {
    // Default is i32, _ will tell comiler to not worry about it
    let x = 1;

    // Defualt f64

    let y = 2.5;

    // add explicit type
    let y: i64 = 45454545454;

    let a1 : char = 'a';

    // find max size of types
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);



}