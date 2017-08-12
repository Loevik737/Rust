/*
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

extern crate time;
use time::PreciseTime;

fn main() {
    let start = PreciseTime::now();
    println!("{:?}", triplet_product());
    let end = PreciseTime::now();
    println!("Used: {} seconds", start.to(end));
}

fn triplet_product() -> u32 {
    let mut product:u32 = 0;
    let mut a:u32 = 0;
    let mut b:u32 = 0;
    let mut c:u32 = 0; 
    for 500..0{
        if
    }
    product
}
