extern crate rand;//vi sier til Rust at vi vil ta i bruk Rand fra Cargo.toml filen
use std::io;//henter nyttige io funksjoner fra biblioteket
use std::cmp::Ordering;//henter nyttige sammenligningsfunksjoner fra biblioteket
use rand::Rng;//Vi sier vi vil bruke metoder fra Rng i rand

fn main() {//lager en funksjon main uten argumenter
    println!("Guess the number");//skriver ut tekstem

    let secret_number = rand::thread_rng().gen_range(1,101);//lager et tilfeldig tall fra 1-100

    loop {//looper til break blir kalt

        println!("Type the number you want to guess:");//skriver ut teksten

        let mut guess = String::new();// let = sier at vi vil binde sammen en variabel til en verdi
                                      // mut = sier at verdien den peker på kan endres
                                      //Srring::new() = lager en ny tom string
        io::stdin().read_line(&mut guess).expect("Failed to read the numder!");//io::stdin().read_line tar inn linjen du skriver
                                                                               //(&mut guess) av en eller annen grun er det ikke
                                                                               //nokk å si at guess en mutable når du deklarerer den
                                                                               //du må også si at den er det når du gir den som arg
                                                                               //i read_line
                                                                               //expect("") = om den feiler gir den en melding som output
        let guess: u32 = match guess.trim().parse() { Ok(num) => num,Err(_) => continue,};//gjør inputen din om til et positivt 32 bit heltall

        println!("You guessed : {} ", guess);//prints out the wariable in the place where {} is

        match guess.cmp(&secret_number) { // sammenligner guess med det du gir cmp som argument
               Ordering::Less    => println!("Too small!"),
               Ordering::Greater => println!("Too big!"),
               Ordering::Equal   => {
                   println!("You win!");
                   break;
               }
           }
      }
}
