/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
Prime nr 100000 is 1299709, used 59 minutes.
*/
extern crate time;
use time::PreciseTime;
fn main() {
    let start = PreciseTime::now();

    println!("Prime nr {0} is {1} ",1001,nth_prime(1001));

    let end = PreciseTime::now();
    println!("Used: {} seconds", start.to(end));
}
//find the nth prime for where n > 2
fn nth_prime(x:u32) -> u32{
    let mut current_prime:u32 = 3;
    let mut prime_nr:u32 = 1;
    let mut check_index = 3;
    while prime_nr < x {
        for i in 2..check_index{
            if check_index % i == 0{
                break;
            }
            if i == check_index -1{
                current_prime = check_index;
                prime_nr += 1;
            }
        }
        check_index += 2;
    }
    return current_prime;
}
