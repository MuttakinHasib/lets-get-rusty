use colored::Colorize;
use rand::Rng;
use std::{cmp::Ordering, io};

pub fn generate_secret_number() -> u32 {
    rand::rng().random_range(1..=5)
}

pub fn get_user_guess() -> u32 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => get_user_guess(),
    }
}

pub fn check_guess(guess: u32, secret_number: u32) -> bool {
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("{}", "Too small!".red());
            false
        }
        Ordering::Greater => {
            println!("{}", "Too big!".red());
            false
        }
        Ordering::Equal => {
            println!("{}", "You win!".green());
            true
        }
    }
}

pub fn play_game() {
    println!("Guess the number!");
    let secret_number = generate_secret_number();

    loop {
        println!("Please input your guess: ");
        let guess = get_user_guess();
        println!("You guessed: {guess}");

        if check_guess(guess, secret_number) {
            break;
        }
    }
}
