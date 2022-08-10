# learning-rust
repo which is used to learn rust

## Compiling & Executing

* `rustc` compiles the `main.rs` to an executable binary 
* `rustc` is suited for simple programs
* to manage options and dependecies `cargo` is better suited than `rustc`

## General

* in rust it is common to structure a program around the data they handle

## Syntax 

* `fn main() {}` is always the first function that runs in every executable rust programm
* indentation is always 4 `spaces` not `tabs`
* `println!` calls a rust macro, this is marked by the `!` at the end, if the `!` would be missing, it would be a functuin call
* a semicolon `;` end an expression in rust
* `("{}")` is used to print a variable

## Packages

* `PathBuf` from `std::path` is also a string, but for file system paths an works cross-platform

