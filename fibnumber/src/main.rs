//Author: AlbinoGazelle
//Purpose: Calculate nth fibonacci number
fn fib(n: u32) -> u32{
    if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    println!("{}",fib(5));
}
