use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("\n\nWelcome to the guessing game!\n\n");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut points: u32 = 0;
    loop{
        points += 1;
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number.");
                continue;
            }
        };

        match guess.cmp(&secret_number){
            Ordering::Greater => println!("Hint: It's smaller\n"),
            Ordering::Less => println!("Hint: It's bigger\n"),
            Ordering::Equal =>{
                println!("Congrats! You won!\n\nYou guessed: {points} times");
                break;
            }
        }
    }
}
