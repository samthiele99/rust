struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color {

    fn new(red: u8, last: u8) -> Color {
        Color {
            red:red,
            green:last,
            blue: 0
        }
    }
}

//tuple struct

struct Color2(u8,u8,u8);

pub fn run() {
    let mut c = Color {red: 255, green: 0, blue: 0};

    c.red = 200;
    
    println!("{} {} {}", c.red, c.green, c.blue);


    let mut c1 = Color2(255,0,0);

    c1.0 = 200;
    
    println!("{} {} {}", c1.0, c1.1, c1.2)

}