use std::io;
use std::cmp::Ordering;

use rand::Rng;


fn main() {
    println!("Welcome to the guessing game!");

    //loop{
        let mut guess = String::new();

        let secret_number = rand::thread_rng().gen_range(1..=100);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        
        println!("You guessed {guess}.")

    //}


    

}
