use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    println!("{}", args.pattern);
    println!("{}", args.path.display());

    // read file from given path
    let input = File::open(&args.path);

    // check if the file was able to be read
    // and unwrap it alread
    // throw error in Err case
    let content = match input {
        Ok(content) => content,
        Err(error) => {
            return Err(error.into());
        }
    };

    println!("File content:\n {:?}", content);

    let reader = BufReader::new(content);

    // print each line of file which contains the pattern
    let mut check: String;

    for line in reader.lines() {
        check = line.unwrap();

        if check.contains(&args.pattern) {
            println!("{}", check);
        }
    }

    // return at last
    Ok(())
}
