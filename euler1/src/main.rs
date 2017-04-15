extern crate time;
use time::PreciseTime;

fn main() {
    let start = PreciseTime::now();
    println!("Sum of even fib numbers: {}",fib_even_sum(34));
    let end = PreciseTime::now();
    println!("DP: {} seconds", start.to(end));
}

fn fib_even_sum(x:usize) -> u64{
        const X: usize = 93;// u64 does not fitt a bigger fib number than fib(93)
        let mut array: [u64; X] = [0; X];
        let mut sum: u64 = 0;
        array[0] = 1;
        array[1] = 1;
        for i in 2..x {
            array[i] = array[i - 1] + array[i - 2];
            if array[i] - (2*(array[i]/2)) == 0{
                sum += array[i];
            }
        }
        return sum;
}
