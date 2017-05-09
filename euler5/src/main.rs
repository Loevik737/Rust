/*
The sum of the squares of the first ten natural numbers is,

12 + 22 + ... + 102 = 385
The square of the sum of the first ten natural numbers is,

(1 + 2 + ... + 10)2 = 552 = 3025
Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/
extern crate time;
use time::PreciseTime;
fn main() {
    let start = PreciseTime::now();

    println!("Sum: {} ", square_difference(100));

    let end = PreciseTime::now();
    println!("Used: {} seconds", start.to(end));
}

fn square_difference(upper_limit: u32) -> u32 {
    let mut sum_of_squears:u32 = 0;
    for i in 1..(upper_limit + 1){
        sum_of_squears += (i as u32).pow(2);
    }
    ((upper_limit*(upper_limit + 1))/2).pow(2) - sum_of_squears
}
