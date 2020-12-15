//Author: AlbinoGazelle
//Purpose: Print the "Twelve Days of Christmas" carole.
fn num_day(n: i32){
    match n {
        1 => print!("first "),
        2 => print!("second "),
        3 => print!("third "),
        4 => print!("fourth "),
        5 => print!("fifth "),
        6 => print!("sixth "),
        7 => print!("seventh "),
        8 => print!("eighth "),
        9 => print!("nineth "),
        10 => print!("tenth "),
        11 => print!("eleventh "),
        12 => print!("twelfth "),
        _ => print!(""),
    }
}

fn items(n: i32){
    let item = match n {
        1 => "A partridge in a pear tree",
        2 => "Two turtle doves, and",
        3 => "Three french hens",
        4 => "Four calling birds",
        5 => "Five golden rings",
        6 => "Six geese a-laying",
        7 => "Seven swans a-swimming",
        8 => "Eight maids a-milking",
        9 => "Nine ladies dancing",
        10 => "Ten lords a-leaping",
        11 => "Eleven pipers piping",
        12 => "Twelve drummers drumming",
        _ => "",
    };
    println!("{}",item);
}


fn main() {
    for number in 1..13{
        print!("On the ");
        num_day(number);
        println!("day of Christmas my true love gave to me:");
        for days in (1..(number + 1)).rev(){
            items(days);
        }
    }
}
