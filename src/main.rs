use std::fs;

fn main() {
    //Try to read a file that doesn't exist.

    let contents = fs::read_to_string("secret_file.txt")
        .expect("Mission log missing! We can't find the secret file! What are we gonna do? What are we gonna do?");
    println!("The secret file contains the following: {}", contents);
    
    
    //let result = fs::read_to_string("non_existent_file.txt")
    //    .expect("The file! I can't find it! What are we gonna do? What are we gonna do?");
    //println!("The file contains: {}", result);
}
