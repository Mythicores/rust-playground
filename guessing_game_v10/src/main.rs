use std::cmp::Ordering;
use std::io;

use rand::Rng;



fn main() {
    println!("Welcome to the guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number: {secret_number}");
    loop{
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Greater => println!("That's too big!"),
            Ordering::Less => println!("That's too small!"),
            Ordering::Equal =>{
                println!("You got it! Congrats!");
                break;
            }
        }
    }
}
