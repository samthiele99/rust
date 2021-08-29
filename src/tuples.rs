// Max tuple length 12
pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);

    println!("{} {} {}", person.0)
}