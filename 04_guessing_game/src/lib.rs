use std::io;
use rand::Rng;

// function which greets the user
// and explains the user interactions
pub fn greeting() {
    println!("Guess the number!");
    println!("Please input your guess");
}

// function to process the user input
// and writes input into guess variable
pub fn process_input() -> Result<String, String> {
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // if guess is empty return Err
    if &guess.chars().count() < &2 {
        Err("No guess given".to_string())
    } else {
        Ok(guess)
    }
}

// generate a random number
pub fn generate_secret_number() -> i32 {
    rand::thread_rng().gen_range(1..=10)
}
