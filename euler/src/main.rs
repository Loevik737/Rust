extern crate time;
use time::PreciseTime;

fn main() {

    let start = PreciseTime::now();
    let mut sum:u32 = 0;
    for value in 1..1000 {
        let n:f32 = 3.0;
        let n1:f32 = 5.0;
        let a:f32 = value as f32;
        if a -(n*(a/n).floor()) == 0.0 || a -(n1*(a/n1).floor()) == 0.0 {
            sum += value;
        }
    }
    let end = PreciseTime::now();
    println!("Sum: {}", sum );
    println!("{} seconds", start.to(end));
}
