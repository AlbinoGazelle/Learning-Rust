fn main() {
    /* let x = 5; //this is a statement

    let y = { //this block evalutes at 4. 
        let x = 3;
        x + 1 //notice no semicolon, adding a semicolon to an expression makes it a statement (which will not return a value)
    }; */
    let x = plus_one(5);

    println!("The value of x is: {}", x); //will print 6 as x
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

/* fn five() -> i32 { //this function will return 5
    5
} */

/* fn another_function(x: i32, y:i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
} */