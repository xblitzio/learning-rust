use std::io;

fn main() {
    println!("Guess the number!");
    println!("Pls input your number: ");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line.");

    println!("You guessed: {number}");
}