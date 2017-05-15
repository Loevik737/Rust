/*
The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.

73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450

Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?

*/
extern crate time;
use time::PreciseTime;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
fn main() {
    let start = PreciseTime::now();

    let path = Path::new("digits.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => println!("The lardgest product py {0} adjacent digits is: {1}", 13, adjacent_product(s,13) ),
    }
    let end = PreciseTime::now();
    println!("Used: {} seconds", start.to(end));
}

fn adjacent_product(number:String, length:u32) -> u64{
    let mut end:u32 = length;
    let mut lardgest_product:u64 = 0;
    let mut current_product:u64 = 1;
    while end < (number.len() as u32){
        current_product = 1;
        for i in (end - length)..end{
            current_product *= number.chars().nth(i as usize).unwrap().to_digit(10).unwrap() as u64;
        }
        if current_product > lardgest_product {
            lardgest_product = current_product;
        }
        end += 1;
    }
    lardgest_product
}
