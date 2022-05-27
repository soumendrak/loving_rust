use std::io;

fn main() {
    let mut word = String::new();
    while word.trim() != "rust" {
        println!("What is the secret word?");
        word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");
    }
}
