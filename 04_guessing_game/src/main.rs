fn main() {
    // greetings for the user
    guessing_game::greeting();

    // create variable for the user guess
    let guess: Result<i32, String>;

    // guess will be one of three variants
    guess = guessing_game::process_input();

    // generate secret number
    let secret_number = guessing_game::generate_secret_number();

    match guess {
        Ok(value) => {
           println!("your guess is {}", value);
           println!("secret number is {}", secret_number );
        },
        Err(error) => eprintln!("ERROR: {}", error),
    }
}
