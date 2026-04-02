use std::io;

use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed {guess}");
    println!("The secret number is {secret_number}");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("That's not a number! Please try again!\n");
            continue;
        }
    };

    match guess.cmp(&secret_number){
        Ordering::Greater => println!("Too big! Try again!\n"),
        Ordering::Less => println!("Too small! Try again!\n"),
        Ordering:: Equal => {
            println!("You got it! Congratulations! Five stars!");
            break;
        }
    }

    }
}
