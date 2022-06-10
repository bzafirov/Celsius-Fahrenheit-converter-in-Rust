// My solution to the Celsius/Fahrenheit convertor exercise

use std::io;

fn main() {
println!("What would you like to convert to?");
println!("Input 'C' to convert Fahrenheit to Celsius or 'F' to convert Celsius to Fahrenheit.");
    let mut convert_to = String::new();

    io::stdin().read_line(&mut convert_to).expect("Failed to read line.");
    let convert_to = convert_to.trim();

    if convert_to == "f" {
        cel_to_fh() // calls the function to convert Celsius to Fahrenheit
    } else if convert_to == "c" {
        fh_to_cel() // calls the function to convert Fahrenheit to Celsius
    } else {
        println!("Command not understood. You can only input 'C' or 'F'");
    }
}


// Function to convert Celsius to Fahrenheit
fn cel_to_fh(){
    println!("Converting Celsius to Fahrenheit. Please input a number");

loop {
let  mut cel = String::new();

io::stdin().read_line(&mut cel)
.expect("That's not a number!");

let cel: f32 = match cel.trim().parse() {
    Ok(cel) => cel,
    Err(_E) => {
        -1.0
    }
};

// string with formula to convert Celsius to Fahrenheit and assign value to fh 
let fh = cel * 9.0 / 5.0 + 32.0;
println!("In Fahrenheit, That's {}", fh);
    }
}


// Function to convert Fahrenheit to Celsius
fn fh_to_cel(){
    println!("Converting Fahrenheit to Celsius. Please input a number");

loop {
let mut fh = String::new();

io::stdin().read_line(&mut fh)
.expect("That's not a number!");

let fh: f32 = match fh.trim().parse() {
    Ok(fh) => fh,
    Err(_E) => {
        -1.0
    }
};

// string with formula to convert Fahrenheit to Celsius and assign value to cel 
let cel = (fh - 32.0) * 5.0 / 9.0;
println!("In Celsius, That's {}", cel);
    }
}