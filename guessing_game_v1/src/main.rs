use std::io;
use rand::Rng;
use rand::thread_rng;

fn main() {
    println!("\nWelcome to the guessing game!\n");
    println!("Write your guess below!:\n");
    let mut guess = String::new();
    io::stdin() //stdin: Under the "io" module. Allows us to interact with the user for input.
        .read_line(&mut guess) //
        .expect("Failed to read line.");
    let secret_number = thread_rng().gen_range(1..=100);
    println!("\nThe secret number is {secret_number}.");
    
}
