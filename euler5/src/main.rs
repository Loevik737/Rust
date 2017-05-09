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

    println!("Smalles number divisible by all numbres from 1 to ");

    let end = PreciseTime::now();
    println!("Used: {} seconds", start.to(end));
}
