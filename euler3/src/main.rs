/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/
extern crate time;
use time::PreciseTime;
fn main() {
    let start = PreciseTime::now();

    //lardgest_palindrome();
    println!("{:?}", lardgest_palindrome() );

    let end = PreciseTime::now();
    println!("Used: {} seconds", start.to(end));
}

fn palindrome(mut n: f64) -> bool {
    let mut num:f64 = 0.0;
    let original:f64 = n;
    let mut digits:f64 = (n.to_string().len() +1 ) as f64;
    let base:f64 = 10.0;
    let mut decimal_number:f64 = 0.0;
    let mut digit: f64 = 0.0;
    if n > 9.0{
        for i in 1..digits as usize{
            decimal_number = n / base;
            digit = n % base;
            num = num + digit * base.powf(digits - i as f64);
            n = (n - digit)/10.0;
        }
        return (num / 10.0 == original);
    }
    original == num
}

fn lardgest_palindrome() -> f64{
    let mut lardgest:f64 = 0.0;
    let mut x:f64 = 0.0;
    let mut a:usize = 0;
    let mut b:usize = 0;
    for i in 100..1000{
        for j in 100..1000{

            x = (i*j) as f64;

            if palindrome(x) && x > lardgest{
                lardgest = x;
                a = i;
                b = j;
            }
        }
    }
    println!("a: {0}, b: {1}", a, b );
    lardgest
}
