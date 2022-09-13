use std::io;

// function which greets the user
// and explains the user interactions
pub fn greeting() {
    println!("Guess the number!");
    println!("Please input your guess");
}

// enum for return type og process_input function
#[derive(Debug)]
pub enum UserInput {
    String(String),
    Number(i32),
    Float(f32),
}

// function to process the user input
// and writes input into guess variable
pub fn process_input() -> UserInput {
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    UserInput::String(String::from(guess))
}
