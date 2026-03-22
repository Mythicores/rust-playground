use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("\nWelcome to the guessing game!\n");
    println!("Write your guess below!:\n");

    
    let mut guess = String::new(); //Creating variable for user input


    io::stdin() //stdin: Under the "io" module. Allows us to interact with the user for input.
        .read_line(&mut guess) //
        .expect("Failed to read line.");

    let guess: u32 = guess.trim().parse().expect("Please type a number");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("\nThe secret number is {secret_number}.");
    match guess.cmp(&secret_number){
        Ordering::Less => println!("That's too low!"),
        Ordering::Greater => println!("That's too high!"),
        Ordering::Equal => println!("You got it!"),
    }
    
}
