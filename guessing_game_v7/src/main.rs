use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("\n\nWelcome to the guessing game!\n\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut points: u32 = 0;
    loop{
        points += 1;
        println!("Enter your guess below:\n");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Greater => println!("Smaller!"),
            Ordering::Less => println!("Bigger!"),
            Ordering::Equal => {
                println!("You got it!");
                println!("Number of guesses: {points}");
                break;
            }
        }
    }
}
