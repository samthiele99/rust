pub fn run() {
    // Print to console
    println!("Hello from the print file");

    // Can only print strings
    // Basic formatting
    println!("{} is from {}","Sam", "Mass");

    // Postional arguments 

    println!("{0} is from {1} and {0} like to {2}", "Sam", "Mass", "code");
    println!("{name} likes {activity}",name="Sam", activity = "bball");


    //Placeholder traits

    println!("Binary: {:b}, Hex {:x} Octal: {:o}", 17, 17, 17);

    println!("{:?}", (12,true,"hello"));


}