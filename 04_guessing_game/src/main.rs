use std::num::ParseIntError;

fn main() {
    // greetings for the user
    guessing_game::greeting();

    // create variable for the user guess
    let mut guess: Result<i32, ParseIntError>;

    // generate secret number
    let secret_number = guessing_game::generate_secret_number();

    // creating a loop, to get as much guesses as needed
    loop {
        // guess will be one of three variants
        guess = guessing_game::process_input();

        match guess {
            Ok(value) => {
                println!("your guess is {}", value);

                // give hint about the guess
                match guessing_game::compare_numbers(value, secret_number) {
                    Ok(message) => {println!("{}", message)},
                    Err(error) => eprintln!("GUESS WRONG: {}", error),
                }
            }
            Err(error) => eprintln!("ERROR: {}", error),
        }
    }
}
