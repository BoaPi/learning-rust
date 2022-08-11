#![allow(unused)]

use clap::Parser;

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
    println!("{}",  args.path.display());

    /// read file from given path
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");

    /// print each line of file
    for line in content.lines() {
        println!("{}", line);
    }
}
