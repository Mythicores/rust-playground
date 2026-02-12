use std::fs;

fn main() {
    let contents = fs::read_to_string("secret_file.txt")
        .expect("Secret log vanished mid-mission!");
    
    println!("The secret file says: {}", contents);
}