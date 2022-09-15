fn main() {
    // greetings for the user
    guessing_game::greeting();

    // create variable for the user guess
    let guess: Result<String, String>;

    // guess will be one of three variants
    guess = guessing_game::process_input();

    match guess {
        Ok(value) => println!("your guess is {}", value),
        Err(error) => println!("ERROR: {}", error),
    }
}
