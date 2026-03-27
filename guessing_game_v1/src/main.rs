use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("\nWelcome to the guessing game!\n");
    
    let mut guess_amount = 0;
    let secret_number = rand::thread_rng().gen_range(1..=100);    

    loop{
        println!("Write your guess below!:\n"); //Gotta make sure it prompts them for the guess each time.
        let mut guess = String::new(); //Creating variable for user input
        io::stdin() //stdin: Under the "io" module. Allows us to interact with the user for input.
            .read_line(&mut guess) //
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("That's not a number, please try again!");
                continue;
            }
        };
        guess_amount = guess_amount +1;
        println!("You guessed: {guess}");
        
        println!("\nThe secret number is {secret_number}.");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("That's too low!"),
            Ordering::Greater => println!("That's too high!"),
            Ordering::Equal => { 
                println!("You got it!\n\nThanks for playing!\n\nYou guessed {guess_amount} times!");
                break;
            }
        }
    }
    
}
