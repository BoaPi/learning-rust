struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    // say hell to the user
    println!("Hello, user");

    // parse cli argumrnent
    // 1st argument - text pattern to look for
    // 2nd argument - path to file, where the test pattern is used
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2). expect("no path given");

    // print both received arguments to the stdout
    println!("{pattern}");
    println!("{path}");

   // putting the received argurments in variable of type Cli
   let args = Cli {
    pattern: pattern,
    path: std::path::PathBuf::from(path),
   };
}
