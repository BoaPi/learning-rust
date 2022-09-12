use std::io;

fn main() {
    // greetings for the user
    greeting();

    // create mutuable variable for the user guess
    let mut guess: String = String::new();

    process_input(&mut guess);

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
fn process_input(guess: &mut String) {
    io::stdin()
        .read_line(guess)
        .expect("Failed to read line");
}
