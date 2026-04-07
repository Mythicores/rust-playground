use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut points: u32 = 0;

    loop{
        let mut guess = String::new();
        points = points + 1;
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");


        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Hey! That's not a number! Guess again!");
                continue;
            },
        };
        println!("You guessed: {guess}");
        println!("The secret number is: {secret_number}");

        match guess.cmp(&secret_number){
            Ordering::Greater => println!("That's too big!"),
            Ordering::Less => println!("That's too small!"),
            Ordering::Equal => {
                println!("You got it! Congrats! You guessed {points} times!");
                break;
        }
        }

    }
}
