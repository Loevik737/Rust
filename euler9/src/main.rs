
/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/
extern crate time;
use time::PreciseTime;
fn main() {

    let start = PreciseTime::now();
    println!("The sum of primes under 2 million:{}",sum_primes(2000000_u64));
    let end = PreciseTime::now();
    println!("Used: {} seconds", start.to(end));
}

fn sum_primes(limit:u64) -> u64{
    let mut i:u64 = 3;
    let mut sum:u64 = 2;
    while i < limit {
        sum += is_prime(i);
        i += 2;
    }
    sum
}

fn is_prime(check_index:u64) -> u64{
    let mut i:u64 = 3;
    while i < check_index/2{
        if check_index % i == 0{
            return 0
        }
        i += 2;
    }
    check_index
}
