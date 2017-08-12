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
    println!("Triplet and product[a,b,c,prod] where a+b+c = {0}: {1:?}",1000, pythagorean_triplet(1000_f64));

    let end = PreciseTime::now();
    println!("Used: {} seconds", start.to(end));
}

fn pythagorean_triplet(n:f64) -> Vec<Vec<f64>>{
    let mut a = 0.0;
    let mut b = 0.0;
    let mut result = Vec::new();
    for i in 2..(((n/2.0).floor()/2.0) as i32){
        a = n*((n/2.0) - (i as f64))/(n-(i as f64));
        if a > 0.0 && a - a.floor() == 0.0  {
            b = i as f64;
            result.push(vec![a,b,n-a-b,a*b*(n-a-b)]);
        }
    }
    result
}
