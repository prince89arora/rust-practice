use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn run() {
    let random_num = rand::thread_rng()
        .gen_range(1, 10);

    println!("Guess the number.....!");
    println!("Please enter your number:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read guessed number..");

    let guess_int: u32 = guess.trim().parse()
        .expect("Unable to parse guessed input...");

    match guess_int.cmp(&random_num) {
        Ordering::Less => { println!("low guess...") },
        Ordering::Equal => { println!("correct guess...") },
        Ordering::Greater => { println!("High guess...") }
    }
}