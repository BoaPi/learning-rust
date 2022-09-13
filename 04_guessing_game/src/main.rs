fn main() {
    // greetings for the user
    guessing_game::greeting();

    // create variable for the user guess
    let guess: guessing_game::UserInput;
    guess = guessing_game::process_input();

    println!("You guessed: {:?}", guess);
}
