//to compile use:
//rustc converter.rs -A warnings
use std::io;
fn main() {
    let mut input_text = String::new();
    println!("Write a 32 bit integer: ", );
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read your input, restart the program..");
    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => println!("Your integer input: Des: {} , Hex: {:x} , Bin: {:b}", i,i,i),
        Err(..) => println!("This was not an integer huh?: {}", trimmed)
    };
}
