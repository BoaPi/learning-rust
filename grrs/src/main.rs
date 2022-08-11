#![allow(unused)]

use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("{}", args.pattern);
    println!("{}", args.path.display());

    /// read file from given path
    /// let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let f = File::open(&args.path);
    let mut reader = BufReader::new(f);

    let content = reader.read();

    /// print each line of file
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
