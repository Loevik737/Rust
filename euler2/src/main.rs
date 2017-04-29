/*
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/
extern crate time;
use time::PreciseTime;
fn main() {
    let start = PreciseTime::now();

    let x:u64 =600851475143;

    println!("Lardgest prime factor is: {} ", prime_factors(x));

    let end = PreciseTime::now();

    println!("Used: {} seconds", start.to(end));
}

fn prime_factors(mut x:u64) -> u64{
    let mut factor:u64 = 0;
    let mut d:u64 = 2;
    while x > 1 {
        while x % d == 0{
            factor = d;
            x = x / d;
        }
        d = d + 1;
        if d*d > x{
            if x > 1{
                factor = x;
            }
            break;
        }
    }
    return factor;
}
