use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Welcome to the guessing game! Version 6");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut points: u32 = 0;

    loop{
        points += 1;
        println!("Write your guess below:\n\n");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number, please try again.");
                continue;
            }
        };
        match guess.cmp(&secret_number){
            Ordering::Greater => println!("That's too big!"),
            Ordering::Less => println!("That's too small!"),
            Ordering::Equal => {
                println!("You got it! You guessed {points} times");
                break;
            }
        }
    }
}
