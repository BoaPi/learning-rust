use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::num::ParseIntError;

// function which greets the user
// and explains the user interactions
pub fn greeting() {
    println!("Guess the number between 1 and 10!");
    println!("Please input your guess");
}

// function to process the user input
// and writes input into guess variable
pub fn process_input() -> Result<i32, ParseIntError> {
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // trim whitespaces and convert back to String
    return guess.trim().parse::<i32>();
}

// generate a random number
pub fn generate_secret_number() -> i32 {
    rand::thread_rng().gen_range(1..=10)
}

// compare two numbers
// return to low, to high or equal message
pub fn compare_numbers(guess: i32, secret: i32) -> Result<String, String> {
    match guess.cmp(&secret) {
        Ordering::Less => Err("To Small".to_string()),
        Ordering::Greater => Err("To Big".to_string()),
        Ordering::Equal => Ok("You Win!!!".to_string()),
    }
}
