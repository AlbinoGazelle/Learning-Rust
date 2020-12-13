fn main() {
    /* let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //break out of the loop and return counter * 2
        }
    }; //end the statement

    println!("The result is {}", result); */

    /* let mut number = 3;

    while number != 0 {
        println!("{}!",number);

        number -= 1;
    }
    println!("LIFTOFF!!!"); */

    //very slow way to access an array
    //also error prone, what if the size isnt 5?
    /* let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    } */
    //faster and safer way to access an array
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    //faster and safer way to do the earlier lift off sequence
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
