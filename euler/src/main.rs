/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.
*/
extern crate time;
use time::PreciseTime;

fn main() {

    let start = PreciseTime::now();
    let mut sum:u32 = 0;
    for value in 1..1000 {
        let n:f32 = 3.0;
        let n1:f32 = 5.0;
        let a:f32 = value as f32;
        if a -(n*(a/n).floor()) == 0.0 || a -(n1*(a/n1).floor()) == 0.0 {
            sum += value;
        }
    }
    let end = PreciseTime::now();
    println!("Sum: {}", sum );
    println!("{} seconds", start.to(end));
}
