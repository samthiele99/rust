pub fn run() {
    greeting("Hey", "Sam");


    let get_sum = add(5,5);

    println!("{}",get_sum);


    let n3 : i32 = 10;
    let add_nums = |n1:i32, n2:i32| n1 + n2 + n3;

    println!("{}",add_nums(2,3))

}



fn greeting(greet: &str, name: &str) {
    println!("{} {}",greet, name)
}


fn add(n1: i32, n2:i32) -> i32{
    n1 + n2
}