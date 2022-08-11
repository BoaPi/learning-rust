# learning-rust
repo which is used to learn rust

## General

* in rust it is common to structure a program around the data they handle

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

* `fn main() {}` is always the first function that runs in every executable rust programm
* indentation is always 4 `spaces` not `tabs`
* `println!` calls a rust macro, this is marked by the `!` at the end, if the `!` would be missing, it would be a functuin call
* a semicolon `;` end an expression in rust
* `("{}")` is used to print a variable
* `PathBuf` does not implement `std::fmt::Display`, therefor to use `println!` on it, we need to call `display()` on the value

## Packages

* `PathBuf` from `std::path` is also a string, but for file system paths an works cross-platform

