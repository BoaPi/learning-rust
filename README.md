# learning-rust
repo which is used to learn rust

## General

* in rust it is common to structure a program around the data they handle
* rust do NOT have exceptions
* all error states are often encoded in the return type
* rust brings in a set of items defined in the standard library, that is in scope of every program, this is called `prelude`
* if a type is not part of the `prelude`, we need to bring it in scope explicit with a `use`statement

## Compiling & Executing

* `rustc` compiles the `main.rs` to an executable binary 
* `rustc` is suited for simple programs
* `cargo` is the build system and package manager of rust
  * builds code
  * downloads libraries
  * building those libraries
* to manage options and dependecies `cargo` is better suited than `rustc`
* dependencies are handled in a `Cargo.toml` file
  * `[package]` handles name, version and edtion
  * edtion indicated the rust language version
  * `[dependencies]` list the project dependencies
* packages in rust are called `crates`

## Syntax 

* variables are defined with `let`
* variables are unmutable by default
* to get a mutable variable it needs to be declared as so: `let mut <name-of-variable>`
* `fn main() {}` is always the first function that runs in every executable rust programm
* a semicolon `;` end an expression in rust
* `!#[allow(unused)]` allows unused variables in code, should only be used in for development
* indentation is always 4 `spaces` not `tabs`
* `println!` calls a rust macro, this is marked by the `!` at the end, if the `!` would be missing, it would be a functuin call
* `("{}")` is used to print a variable
* `PathBuf` does not implement `std::fmt::Display`, therefor to use `println!` on it, we need to call `display()` on the value
* `String.trim_end()` removes trailign whitespaces

### Reading from files

* a `File::open()` not a string, it returns a `Result`
* a `Result` in this case, contains a `String` and an `Error`
* a `Result` is an `enum`
* to handle the enum variants, we can use `match`
* `unwrap()` is the shortcut method to do the same

## Packages

* `PathBuf` from `std::path` is also a string, but for file system paths an works cross-platform
* `std::fs::read_to_string` will read fill to a value
* calling `.lines()` on the return value of `std::fs::read_to_string` will give me an iterable value, where each index is a line
* the result of `.lines()` is a `String`, which has a `.contains()` method, to check if a string contains a certain pattern

