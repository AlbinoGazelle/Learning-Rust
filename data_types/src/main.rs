fn main() {
    //four data types in Rust
    //integers, floating-points, booleans, and characters

    //  Integers
    //  Length    Signed    Unsigned
    //  8-bit     i8        u8
    //  16-bit    i16       u16
    //  32-bit    i32       u32
    //  64-bit    i64       u64
    //  128-bit   i128      u128
    //  arch      isize     usize
    //  arch types are computer specific (64 bit computers isize = 64, 32 bit computers isize = 32 good for portability?)

    //  Floating-Point
    //  Rust has two types of floating points, f32 and f64. f64 is roughly the same speed but more precise so it's the default
    let x = 2.0; // f64
    let y: f32 = 3.0; //f32
    println!("x is: {} y is : {}", x, y);

    // Numeric Operations

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("sum: {} difference: {} product: {} quotient: {} remainder: {}", sum, difference, product, quotient, remainder);

    //  Boolean Types
    //  booleans are either true of false, they are one byte in size.

    let t = true;

    let f: bool = false; //with expicit type annotation

    println!("t: {} f: {}", t, f);

    // Character Type
    // Four bytes in size and are in unicode. 
    // They are specified with single quotes, not double

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("c: {} z: {} heart cat: {}", c, z, heart_eyed_cat); //emoji is pretty cool! (not sure if C had this)

    // Compound types can group values into one type. Rust has two, tuples and arrays
    
    // Tuples
    // Tuples have a fixed length, once declared they cannot grow or shrink in size
    // Way of grouping together values into one compound type

    // Since tup is considered a single element, how do we get individual values?
    let tup = (500, 6.4, 1);
    // Solution: Pattern matching. x is 500, y is 6.4, z is 1

    let (x, y, z) = tup; //this is super cool!

    // What we just did is called Destructuring because it breaks the tuple into three parts
    println!("The value of x is: {} The value of y is: {} The value of z is: {}", x, y, z);

    // We can also access a specific value by using a period.
    let k = tup.1; //k = 500
    println!("k is: {}", k);

    // OR
    let h: (i32, f64, u8) = (500, 6.4, 1);
    let q = h.0; //q = 500 (wonder if we can do this in a loop with index? access x.i maybe?)

    println!("q is: {}",q);

    // Arrays
    // All elements in an array must be the same type
    // Arrays in Rust have fixed lengths

    let array = [1, 2, 3, 4, 5];
    println!("First value is: {}",array[0]);

    // Can also declare them strictly like this
    let strict_array : [i32; 5] = [1, 2, 3, 4, 5];
    println!("First value in strict: {}",strict_array[0]);

    // Rust will not let you access an invalid array index
}
