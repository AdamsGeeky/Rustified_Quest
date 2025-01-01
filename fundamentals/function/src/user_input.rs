use std::io;

pub fn user_name()-> String {
    println!("Please Enter Your Full Name : ");

    // Create Mutable String
    let mut full_name = String::new();

    // Read the user input 
    io::stdin()
        .read_line(&mut full_name)
        .expect("Fail to Read inpt");

    // trim any whitespace

    full_name.trim().to_string()
}