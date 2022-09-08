use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader, Write};

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

    // if patten is empty return nothing and skip searching
    if str::is_empty(&args.pattern) {
        writeln!(handle, "No pattern provided")?;
    } else {
        // read file from given path
        let input = File::open(&args.path);

        // check if the file was able to be read
        let content =
            input.with_context(|| format!("could not read file `{}`", &args.path.display()))?;

        writeln!(handle, "File content:\n {:?}", content)?;

        let reader = BufReader::new(content);

        // check reader content for matching pattern
        let findings: String = grrs::find_matches(reader, &args.pattern);

        // print result
        writeln!(handle, "{}", findings)?;

        // return at last
        writeln!(
            handle,
            "Search of {:?} in file {:?} done",
            args.pattern, args.path
        )?;
    };

    Ok(())
}
