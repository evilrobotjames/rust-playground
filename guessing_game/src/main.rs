use std::io;

fn main() {
    println!("Guessing game started");

    println!("Enter your guess:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed {guess}");



}

