use std::io; //std io. "std" = standard "io" = input/output
use std::cmp::Ordering; // std:: comparing tool::Ordering. Be sure to import this

use rand::Rng; //Rng is the trait


fn main() {
    println!("Welcome to the guessing game!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // Generate the secret number from rand, not the trait Rng. Use thread_rng, and gen_range.
    loop{
        println!("Write your guess below:\n\n");

        
        let mut guess = String::new(); //Create the new string to hold the user's guess
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        
        println!("You guessed {guess}");
        println!("The secret number is {secret_number}");

        let guess: u32 = match guess.trim().parse(){ // Format: declare variable | use ":" and say the type it's going to be | use match then trim and parse | declare the options
            Ok(num) => num, 
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("That's too big!"),
            Ordering::Less => println!("That's too small!"),
            Ordering::Equal => {
                println!("You got it! Congratulations!\n\n");
                break;
            }
        }

    }


    

}
