/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/
extern crate time;
use time::PreciseTime;
fn main() {
    let start = PreciseTime::now();

    println!("{:?}", lardgest_palindrome(3) );

    let end = PreciseTime::now();
    println!("Used: {} seconds", start.to(end));
}

fn palindrome(mut n: f64, base:f64) -> bool {
    let mut num:f64 = 0.0;
    let original:f64 = n;
    let digits:usize = n.to_string().len() + 1;
    if n > 9.0{
        for i in 1..digits{
            num = num + n % base * base.powf(digits as f64 - i as f64);
            n = (n - n % base)/10.0;
        }
        return num / 10.0 == original;
    }
    original == num
}

fn lardgest_palindrome(factor_length:u32) -> f64{
    let mut lardgest:f64 = 0.0;
    let mut a:usize = 0;
    let mut b:usize = 0;
    let mut x:f64 = 0.0;
    for i in 10_usize.pow(factor_length-1)..10_usize.pow(factor_length){
        for j in i..10_usize.pow(factor_length){
            if j > i{
                x = (i*j) as f64;
                if palindrome(x, 10.0) && x > lardgest{
                    lardgest = x;
                    a = i;
                    b = j;
                }
            }
        }
    }
    println!("a: {0}, b: {1}", a, b );
    lardgest
}
