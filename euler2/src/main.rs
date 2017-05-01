/*
The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143 ?
*/
extern crate time;
use time::PreciseTime;

fn main() {
    let start = PreciseTime::now();

    println!("{:?}",prime(10));

    let end = PreciseTime::now();

    println!("{} seconds", start.to(end));
}

fn prime(x: u64)-> [bool;20]{
    const X:usize = 20;
    let mut array: [bool;X] = [true;X];
    let mut k:u32 = 0;
    for i in 1..(X as f64).sqrt() as u32 {
        if array[i as usize] {
            for j in 2..X as u32 {
                if j == ((i^2) + i*k) {
                    array[j as usize] = false;
                    k+=1;
                };
            };
        };
    };
    return array;
}
