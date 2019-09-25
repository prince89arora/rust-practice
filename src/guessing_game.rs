use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn run() {
    let random_num = rand::thread_rng()
        .gen_range(1, 10);

    println!("Guess the number.....!");

    loop {
        println!("Please enter your number:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read guessed number..");

        let guess: u32 = match guess.trim().parse() {
            Ok( num ) => num,
            Err( _e ) => continue,
        };

        if validate_input(random_num, guess) {
            break;
        }
    }
}

fn validate_input(random_num:u32, input:u32) -> bool {
    let flag: bool;
    match input.cmp(&random_num) {
        Ordering::Less => {
            println!("low guess...");
            flag = false;
        },
        Ordering::Equal => {
            println!("correct guess...");
            println!("--- You Win!! ---");
            flag = true;
        },
        Ordering::Greater => {
            println!("High guess...");
            flag = false;
        }
    }
    return flag;
}