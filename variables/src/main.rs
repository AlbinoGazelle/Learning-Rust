fn main() {
    //constants must have a datatype.
    //const MAX_POINTS: u32 = 100000;

    //this process is called "shadowing" it allows us to "change" immutable variables.
    //x is currently 5
    let x = 5;
    //x is then 5 + 1
    let x = x + 1;
    //x is finally 6 * 2
    let x = x * 2;
    //if we used mut, the variable would be able to be changed after these operations
    //since we shadowed it, the variable is immutable after these operations.
    println!("The value of x is: {}", x);
    
}