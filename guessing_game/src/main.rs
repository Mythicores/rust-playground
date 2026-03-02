//Note! This will not compile! It is being used in the middle of a lesson.

use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");

    println!("Please input your guess.");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number. ");

    println!("You guessed {guess}!");
    println!("The secret number is: {secret_number}");

    //Ideally we'd want a while loop that checks to see if the guessed number equals the secret number.
    //Nope! I was wrong! Using [[Rust/control-flow/match|match]] is more optimal.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You got it! Five stars!"),
    }

}
