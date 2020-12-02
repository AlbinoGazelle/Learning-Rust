use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {

    println!("Guess the number! Enter \"quit\" to exit.");

    let secret_number = rand::thread_rng().gen_range(1, 101); //use rand::Rng to create a random number
    //loop keyword creates an infinite loop (Make sure to add a way to break out!)
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) //read line from stdin() and store inside guess
            .expect("Failed to read line."); //error if we cant read line
        //exit program if we get a quit keyword
        if guess.trim() == "quit"{
            println!("Exiting!");
            break;
        }
        //converting our input into a 32 bit unsigned int to compare to our random number
        //we swapped from an expect, to a match statement. This allows us to handle errors instead of breaking
        //If the result of the conversion is Ok, we return the number, if the result is bad and errors we will continue
        //the _ in Err is a catchall, this means we want to catch ALL errors and not a specific one
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}",guess);
        //this codeblock compares our guess variable to our secret number variable
        //it uses std::cmp::Ordering to display phrases based on the result of the comparison
        //if our guess matches the secret number, we break and exit the program
        //Extra: Add a keyword (quit, exit) that the user can type to exit the program
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
