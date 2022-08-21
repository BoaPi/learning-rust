use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // initialize stdout, locks stdout
    // and create new BufWriter
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    // get all cli arguments
    let args = Cli::parse();

    // read file from given path
    let input = File::open(&args.path);

    // check if the file was able to be read
    let content =
        input.with_context(|| format!("could not read file `{}`", &args.path.display()))?;

    writeln!(handle, "File content:\n {:?}", content)?;

    let reader = BufReader::new(content);

    // print each line of file which contains the pattern
    let mut check: String;

    for line in reader.lines() {
        check = line.unwrap();

        if check.contains(&args.pattern) {
            writeln!(handle, "{}", check)?;
        }
    }

    // return at last
    writeln!(handle, "\nSearch of {:?} in file {:?} done", args.pattern, args.path)?;
    Ok(())
}

fn find_matches(reader, pattern) {
    
} 

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn my_first_unit_test() {
        assert_eq!(answer(), 42);
    }
}
