#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;

fn main() {
    println!("What is your name?");

    let mut name = String::new();
    let greeting: &str = "Nice to meet you";

    io::stdin().read_line(&mut name)
        .expect("Did not receive input");

    println!("Hello {}!, {}", name.trim_end(), greeting);
}
