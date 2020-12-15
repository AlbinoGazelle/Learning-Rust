//function to get the first word in a string using slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}



fn main() {
    //Slices are a way to reference a sequence of elements in a collection rather than the whole collection
    let s = String::from("hello");
    //both of these are the same thing
    let _slice = &s[0..2]; //"he"
    let _slice = &s[..2]; //"he"

    let len = s.len();

    //these are also the same thing. If your slice includes the last byte of the string you can drop the trailing number
    let _slice = &s[3..len]; //"lo"
    let _slice = &s[3..]; //"lo"
    
    //you can also drop both values to take the slice of the entire string
    let _slice = &s[0..len]; //"hello"
    let _slice = &s[..]; //"hello"


    let my_string = String::from("hello world");
    // function works on slices of Strings
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";
    //function also works on slices of string literals
    let _word = first_word(&my_string_literal[..]);
    //also works on string literals since they ARE slices already (this is essentially the same as the above line of code)
    let _word = first_word(my_string_literal);

    //you can also take slices of other data structures like arrays
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3]; //1, 2, 3
}
