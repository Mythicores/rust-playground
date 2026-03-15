use std::io;

fn main() {
    println!("\nWelcome to the guessing game!\n");
    println!("Write your guess below!:\n");
    let mut guess = String::new();
    io::stdin() //stdin: Under the "io" module. Allows us to interact with the user for input.
        .read_line(&mut guess) //
        .expect("Failed to read line.");
    let secret_number = 50; //I can't remember how to use the rand module, I'll check tomorrow.
    println!("The secret number is {secret_number}.");
    
}
