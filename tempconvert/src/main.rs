//Author: AlbinoGazelle
//Purpose: Convert temperatures between Celsius and Fahrenheit
use std::io;
fn main() {
    println!("What are you converting to? (C or F): ");
    let mut temp_check_str = String::new();
    
    io::stdin().read_line(&mut temp_check_str).expect("Failed to read line.");
    
    println!("Enter the value you want to convert: ");
    let mut temp_str = String::new();

    io::stdin().read_line(&mut temp_str).expect("Failed to read line.");
    temp_str.pop(); //remove newline so our output looks better
    if temp_check_str.to_uppercase().trim().to_string() == "C" {
        let temp_int: f64 = temp_str.trim().parse().unwrap(); //convert our string into an int

        let temp_int = (temp_int - 32.0) * 5.0 / 9.0; //do conversions
        println!("{}°F is {:.2}°C ",temp_str,temp_int); //print our result
    } else {
        let temp_int: f64 = temp_str.trim().parse().unwrap();
        let temp_int = (temp_int * 9.0/5.0) + 32.0;
        
        println!("{:.2}°C is {:.2}°F ",temp_str, temp_int);
    }
}
