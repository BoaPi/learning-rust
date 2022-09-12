use std::io;

fn main() {
    // greetings for the user
    greeting();

    // create variable for the user guess
    let guess: String;
    guess = process_input();

    println!("You guessed: {guess}");
}

// function which greets the user
// and explains the user interactions
fn greeting() {
    println!("Guess the number!");
    println!("Please input your guess");
}

// function to process the user input
// and writes input into guess variable
fn process_input() -> String {
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess
}
