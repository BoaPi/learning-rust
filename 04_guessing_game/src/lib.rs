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
    Text(String),
    SimpleNumber(i32),
    FancyNumber(f32),
}

// function to process the user input
// and writes input into guess variable
pub fn process_input() -> Result<UserInput, String> {
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if &guess.chars().count() < &2 {
        return Err("No guess given".to_string());
    } else {
        println!("{:?}", &guess.chars().count());
        return Ok(UserInput::Text(String::from(guess)));
    }
}
