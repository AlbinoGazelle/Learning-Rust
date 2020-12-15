fn main() 
{ //s is not valid here, it's not been declared yet
    //let s = "hello";  //s is valid from this point onward
    let mut s = String::from("hello"); //this creates a String type. Similar to our earlier s but is muttable.
    s.push_str(", world!"); //push_str() appends a literal to a string

    //Example of "moving"
    let s1 = String::from("hello");
    //let s2 = s1;

    //println!("{}, world",s1) //this wont work! because rust has invalidated s1 after we copied to to s2, "moving" it to a different variable.

    //if we wanted to copy the values, we would use the clone() method. (this is an expensive operation)
    let _s2 = s1.clone();

} //the scope is now over, and s is no longer valid.
//Good example with scope annotations
/*
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
*/