# learning-rust
repo which is used to learn rust

## General

* in rust it is common to structure a program around the data they handle
* rust do NOT have exceptions
* all error states are often encoded in the return type
* rust brings in a set of items defined in the standard library, that is in scope of every program, this is called `prelude`
* if a type is not part of the `prelude`, we need to bring it in scope explicit with a `use`statement

### Heap & Stack

* most programming languages do not require to think of `heap`and `stack`
* both are part of the memory that is available to the code
* when a function is called, the values which are passed to the function (including also pointer to data on the `heap`)
  and local variables get pushed into the `stack` and will be popped out 
  when the functiuon is over

#### Stack

* stores data in the order it gets them
* removes the data in the opposite order
* `LIFO` - last in first out principle
* add data to the stack is called **pushing onto the stack**
* removing data from the stack is called **popping off the stack**
* all data stored in the stack must have a known size
* data with unknown size at compile time needs to be stored in the `heap`
* pishing to the `stack`is fast, because the allocator does not to search for a spot

#### Heap

* is less organized than the `stack`
* when data is put into the `heap`, a certain amount of memory is requested
* the memory allocator finds an empty spot, marks it as `in use` and returns a pointer to the data
* the process is called **allocating on the heap**
* because the pointer to the `heap` is a knwon size, it will be stored on the `stack`
* to get the actual data, the pointer needs to be followed into the `heap`
* accessing data in the `heap` is slower than in data in the `stack`, because the pointer needs to be followed

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

### Smart-Pointer

* `Pointer` is a address to a memory location
* `Box<T>` is the most straigforward smart pointer
* a `Box<T>` smart pointer stores data on the `heap`instead of the `stack`
* `Boxes` do not have a performance overhead, but also do not have extra caüabilities
* common uses cases are:
  * a type with an unknown size at compile time and a value of that type should be used
  in a context which requires an exact size
  * when a value should be owned, where it is only needed to know that
  a certain trait is implemented, rahter than a specific type

## Packages

* `PathBuf` from `std::path` is also a string, but for file system paths an works cross-platform
* `std::fs::read_to_string` will read fill to a value
* calling `.lines()` on the return value of `std::fs::read_to_string` will give me an iterable value, where each index is a line
* the result of `.lines()` is a `String`, which has a `.contains()` method, to check if a string contains a certain pattern

