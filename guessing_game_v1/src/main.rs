use std::io;
use rand::Rng;


fn main() {
    println!("\nWelcome to the guessing game!\n");
    println!("Write your guess below!:\n");


    let mut guess = String::new(); //Creating variable for user input


    io::stdin() //stdin: Under the "io" module. Allows us to interact with the user for input.
        .read_line(&mut guess) //
        .expect("Failed to read line.");

    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("\nThe secret number is {secret_number}.");
    
}
