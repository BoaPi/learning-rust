fn main() {
    // greetings for the user
    guessing_game::greeting();

    // create variable for the user guess
    let guess: String;
    guess = guessing_game::process_input();

    println!("You guessed: {guess}");
}
