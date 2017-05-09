/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/
extern crate time;
use time::PreciseTime;
fn main() {
    let start = PreciseTime::now();

    println!("Smalles number divisible by all numbres from 1 to {1} is {0}",smailest_divisible(20), 20);

    let end = PreciseTime::now();
    println!("Used: {} seconds", start.to(end));
}

fn smailest_divisible(mut max_divisible: usize) -> u64{
    let mut divisible:bool = true;
    let mut x:u64 = (max_divisible*2) as u64;
    max_divisible = max_divisible + 1;
    while divisible{
        for i in 3..max_divisible{
            if x % i as u64 != 0{
                divisible = false;
                break;
            }
        }
        if divisible{
            break;
        }
        x = x + 2;
        divisible = true;
    }
    x
}
