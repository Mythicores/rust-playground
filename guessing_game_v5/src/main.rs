use std::cmp::Ordering;
use std::io;

use rand::Rng;


fn main() {

    println!("Welcome to the guessing game! Version 5");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut points = 0;

    loop{

        println!("Write your guess below!");
        
        let mut guess = String::new();
        points += 1;

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number!");
                continue;
            }
        };
        println!("You guessed: {guess}");
        println!("The secret number is {secret_number}");

        match guess.cmp(&secret_number){
            Ordering::Greater => println!("That's too big!"),
            Ordering::Less => println!("That's too small!"),
            Ordering::Equal => {
                println!("That's correct! You win!\n\n You guessed {points} times!");
                break;
            }
        };

    }
}
