use std::io;

pub fn run() {
    println!("Guess the number.....!");
    println!("Please enter your number:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read guessed number..");

    println!("You have guessed: {}", guess);
}