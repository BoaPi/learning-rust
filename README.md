# learning-rust 🦀
repo which is used to learn rust

## General

* it is common to structure a program around the data they handle
* rust do NOT have exceptions
* all error states are often encoded in the return type
* rust brings in a set of items defined in the standard library, that is in scope of every program, this is called `prelude`
* if a type is not part of the `prelude`, we need to bring it in scope explicit with a `use`statement
* `{}` contains the content of a function
* variables are immutable by **default**
* references to variables are also immutable by **default**
* once given a value, a value of a variable won't change
* to make a variable mutable, the `mut` keyword needs to be used
* `//` marks comments until end of line
* `{}` inside `println!()` are placeholders for variables
* `{}` means for the `println!()` macro to use formatting known as **Display**
* **Display** means: output intended for end user consumption
* constants are not allowed to change, they are always immutable
* constants using the keyword `const`
* the type of a constant have to be annotated
* constants can be declared at any scope
* constants are only allowed to be an constant expression, **not** a result of a runtime computation
* constants have a naming convention, `NAME_OF_THE_CONSTANT`, all uppercase with underscores between words
* **shadowing** is the re-declaration of a **immutable** variable 
* **shadowing** is scope related
* **shadowing** is **not** mutability, reassignment only work with `let` keyword
* after reassignment the variable is immutable again
* **shadowing** can be used to change the type of a variable, to reuse the name
* **rust** will never automatically do a **deep** copy
* **rust** do **not** have a `Null` value

## Ownership

* most unique feature of **rust*
* it enables memory safety guarantees without a garbage collector
* **ownership** is a set of rules to govern how the memory gets managed
* if any rule is violated, the program won't compile
* allocated memory will be returned to the **memory allocator**,
  when the variable that owns it is out of scope
* when a variable goes out of scope, rust calls `drop` to return
  the allocated memory the memory allocator
* when a **variable** that includes data on the heap goes out of scope, the related value will be **dropped**
* unless the data was **moved** to another variable

### Ownership rules

* each value in **rust** is an *owner*
* there can only be one owner at a time
* when the owner goes out of scope, the value will be **dropped**

### Ownership of different types

* **integers** are simple values of known, fixed size. Therefor will put into **stack**
  This seems to be the opposite of the `String` behavior, but because all data live in the **stack**, 
  a deep copy or shallow copy is the same.
* **Strings** - when this type get assigned from one variable to another, the **pointer**, **length** and **capacity** 
  gets copied to the new variable. The content on the **heap** stays untouched.
  Also the first variable will be **dropped**. This operation is called **move**.
  Do get a actual **deep** copy of a `String` type, the `clone()` method can be used. This copies not only the **stack**
  data, but also the **heap** data. Therefor each variable has ownership of its own pointer content
* variables which uses `Copy``trait will be still valid after assignment to another variable.
  This trait can only be implemented by type, where the type and no part of it implements the `Drop` trait.
  Rule of thumb: every scalar types and type which requires no allocation or is some form of resource, can implement `Copy` trait. 
  * all **integer** types
  * **boolean** type
  * all ***floating** types
  * **char** type
  * **Tuples** - if it only contains types which can implement `Copy`

### Ownership and Functions

* passing values to functions is similar to assign a value to a variable
* passing will **move** or **copy**
* when functions return a values, ownership is given
* functions can take ownership and give it back

### References

* `reference` is an address to the stored data, but owned by an other variable
* `reference` guarantees a valid value of a particular type for the `reference` lifetime
* `references` are declared with an `&`
* referencing an value of a variable is called `borrowing`
* `references` are immutable by default, you can **not** mutate what you **not** own
* `mutable references` are able to be changed and therefor will mutate the value it borrows
* `mutable references` are restricted to be the only one at the time
* `mutable references` prevents data races at compile time
* `data races` occur when:
  * two or more pointer access the same data at the same time
  * at least one of the pointer writes to the data
  * there is no **sync** to access of the data
* with `scope` it is possible to have multiple `mutable references` but not at the same time
* `immutable references` & `mutable references` at the same time are also not allowed.
  The reason is, an user of an `immutable reference` is not expecting any change of
  the reference. Multiple `immutable references` are allowed.
* `immutable references` will go out of scope when **not** used anymore.
  The ability of the compiler to tell that a reference is **not** used anymore
  before the scopes ends, is called **Non-Lexical-Lifetimes** (NLL)
* pointers which references a memory location that is owned by an other variable
  are called `dangling pointers/references`
* rust guarantees that a `reference` will never be `dangling`
* the compiler will ensure that the data will not go out of scope before the `reference`

### Slice

* `slices` are a kind of `reference`
* `slices` do **not** have ownership of the data
* `slices` are a contiguous sequence of elements in a collection
  and not whole collection
* `string slice` is a `reference` to a part of the String
* `string slice` syntax looks like this; `&s[0..3]`
* there are also other slices than strings slices, e.g. `array slices`
    
## Literal Syntax

* `literals` are tokens used in literal expressions
* byte literal is a single **ASCII** character, e.g `b' '` can be used to check for space in byte

## Data types

* **scalar** and **compound** are a subset of data types
* **scalar** types are single value types
* **scalar** types are `integer`, `floating-point numbers`, `Booleans` and `characters`
* rust has two **compound** types, `Tuple` and `Array`
* **compound** types can hold multiple values
* primitive types do implement `Display` trait by default,
  because there is only one way to print them
* for `structs`there are more possibilities to print them, therefore the
  `Display` can not be implemented by default
* good way to print `structs` is to use the `Debug` trait

### String Types

* type `String` is most common and hold the ownership of the content
  * can be create from a literal string - `String`
  * `String::new` is used to create a new instance of `String`, which is empty
  * `::from` or `::new` namespaces the `from` and `new` functions under the String type
* type `str` is the primitive counterpart of `String`
  * also called **string slice**
  * usually seen in its borrowed form `&str`
  * always valid **UTF-8**
  * by default `immutable reference`
* `String` literals as `let mut s = "some string";` will **not** compile, this is because of:
  * `String literals` are hard coded and the content is known at compile time
  * therefore it lives in the stack
* `String Type` created with `let mut s = String::from("some string");` will compile
  * in order to have mutability, memory will be allocated on the heap
  * this memory needs to be allocated by the **memory allocator** at runtime
  * **pointer reference**, **length** and **capacity** of a `String` is stored in the stack
  * content of the **pointer reference** is stored in the heap
  * **length** is current memory usage in bytes of the content of the `String`
  * **capacity** - tbd

### Integer Types

* 8-bit, 16-bit, 32-bit, 64-bit or 128-bit
* **signed** or **unsigned**
* when it is sure that the number always will be positiv, **unsigned** integers can be used
* **isize** or **usize** indicated dependens on the architecture of the computer the programm is running on
* integers can be written in **decimal**, **hex**, **octal**, **binary** or **bit** (u8 only)
* as a visual separator and ease up reading an `_` can be used, e.g. `1_000`

### Floating-Point Types

* 32-bit or 64-bit
* default is **f64**
* all floating-point numbers are **signed**
* **f32** is single-percision float
* **f64** is double-percision float

### Numeric operations

* addition, subtraction, multiplication, division and remainder

### Boolean Types

* possible states **true** or **false**
* mainly used in conditionals, e.g. **if** statements

### Tuple Types

* is used to group a number of values, with a variety of types
* have fixed length, ones decalred this can not be changed
* declared comma-separated inside of parentheses
* each position has a type, types do not have to be the same
* tuples can be destructured
* elements can also be accessed using a period **.** sign
* starting index is **0**
* **unit** is a special case of tuple, where value and corresponding type are written in `()`

### Array Types

* colletion of multiple values
* all values have the same type
* have a fixed length and number of elements will not change
* useful to allocate data on the `stack`, instead to the `heap` or to have always a fixed number of elements
* initialized with `[i32; 5]` means five elements of type *i32*
* initialized with `[5; 3]` means three times the same element with the same value
* elements can be accessed via `array[0]` - to get the first element
* if an element out of bound is tried to be accessed, **rust** panics
* can only be indexed with **unsigned** numbers

### Character Types

* most primitive **alphabetic** type
* `char` literals are specified with single quotes, e.g. `let c: char = 'c';`
* **4 bytes** in size
* represents a **scalar** unicode value
* accented letters, Chinese, Japanese, and Korean characters, emoji,
  and zero-width spaces are all valid char values in Rust
* range is from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF`

## Functions

* most important is the `main` function, because it is the entry point of the program
* `snake_case` is convention for naming functions
* `fn`keyword is used to define a function
* position of the function definition in one file does not matte, because it is the entry point of the program
* `snake_case` is convention for naming functions
* `fn`keyword is used to define a function
* position of the function definition in one file does not matter
* function signatures **must** declare the type of each argument
* build out of statements, which can have an optional ending in an expression
* can return an value
* no name for return value needed
* return value of a function is the same as the final expression of a function
* returning earlies in a function needs the **return** keyword

## Methods

* `methods`are defined within in the context of a `struct`, `trait object` or `enum`
* first parameter is always **self**
* **self** represents the instance of the `struct` the methods is called on
* `methods` are defined with the **impl** keyword
* multiple **impl** blocks are possible
* `methods` are called with the **dot notation**
* `methods` can have same names as fields of a `struct`, usually this `methods`
  return the value of the field and are called **getters**
  **getters** will not be implemented automatically and are useful to create an
  public API for the `struct`, thus they are read-only
* `&self` in methods signature is short form for `self: &Self`
* **self** can be:
  * taken ownership of --> rare use case, usually if we want to prevent the
    caller fro using the original instance after transformation
  * borrow immutable --> to read data from the struct, but not write to it
  * borrow mutable --> if we want to change the data of the instance
* **rust** has **automatic referencing and dereferencing** for `methods`,
  this works because `methods` have a clear receiver --> **self**. This is either
  `&self`, `&mut self` or `self`
* `functions` which are defined within in **impl** block are called **associated functions**,
    because they are associated with the type
* **associated functions** without the **self** parameter are also possible, they do **not**
  need an instance of the type. For example `String::from`
* **associated functions** without **self** parameter are **not** a `method`.
  Usually used as constructor to create a new instance.
* **associated functions** which construct uses **Self** as return type. The **Self**
  is a alias for the type that appears in the **impl** block
* **associated functions** are called with `::`

### Statement & Expressions

* **statements** are instructions, which do not return a value
* **expressions** are evaluated to resulting in a value
* calling a function, calling a macro and a scope block is an **expression**
* **statements** have a trailing semicolon
* **expressions** do not have trailing semicolon, adding one will it make an statement

## Control Flow

* rust has **if expressions** and **loops**
* **if expressions** needs to evaluate to a **bool**
* no automaatic conversion of types to **bool**
* an **if expression** can be on the right site of a **let**, to the outcome to the variable
* when using this approach, both arms of the **if expression** needs to be the same type
* `match` can be used to compare values against a series of **pattern** and execute code based on it
* **pattern** could be: literal values, variable names, wildcards and other things
* 

### Loops & While

* `loop` keyword runs block of code over and over again
* runs until it will get break
* `Ctrl-c` or `break` keyword, can break the **loop**
* `continue` keyword can be used to skip over remaining code and start next iteration
* **loops** can be used to retry an operation, e.g. checking if a thread if it has completed its job
* values behind the `break` condition will be returned
* **loops** can be nested
* `break` and `continue` will effect the innermost **loop**
* **loop labels** can be used to specify which **loop** should be affected by `break` and `continue`
* `while` is  used when a program should evaluate befor each loop if the
  conditions are still **true**, otherwise break the loop
* `while` loop eleminating a lot of `if`, `else` & `break` nesting, to break a `loop`
* `for` loop can be used to loop through a collection

### Structs or Structures

* custom data type
* package together and name multiple related data, that makes up a meaningful group
* the single pieces of a struct can be different types
* each piece of data has a name
* to access data the **dot notation** can be used, instead of order of the pieces as in `tuples`
* defined with the `struct` keyword
* to use a `struct` an instance with concrete values for all the pieces needs to be created
* to create an instance the name of the `struct` s used `Example_Struct {}` 
* inside the curly brackets **key: value** pairs are used
* if the instance is mutable, the value can be changed with **dot notation**
* only the whole `struct` can be mutable or immutable, **not** specific fields
* update syntax `..<struct_name_to_inherit_from>` can be used to create new structs from other structs
* update syntax can lead to, that the inherited struct can **not** be longer used,
  due to missing **copy trait**. E.g. for `String` types
* `tuple structs` are a special `struct` variant
* `tuple sturcts` have a name compared to usual `tuples`
* `tuple structs` do **not** have named fields like `struct`
* `tuple structs` can also be destructured and **dot notion** with an index to access a piece of data
* `unit like structs` are `structs` that do **not** have any fields and behave similar to `()`
* `unit like structs` are useful when a `trait` should be implemented on a type, but there is 
  no data that should be stored in the type itself

## Standard libraries

* to obtain user input and write output we need the `io` standard lib
* to use `io` it needs to be imported via `std:io`

## Crates

* an executable is a `binary crate`
* an external one, is called `library crate`
  * code is intended to used in other programs
  * can't be executed on its own
* external `library crates` are used to extend the functionality of the program
* external `library crates` are managed in the `Cargo.toml` file of the project
* **crates.io** is the site where **crates** gets published
* `Cargo.lock` file is used to pin down every used version of a crate for the project
* `Cargo.lock` ensures that every build is the same
* `cargo update` can update dependencies

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
* not all variables can just be printed, therefore we can use the **debug representation**, `{:?}`
* `PathBuf` does not implement `std::fmt::Display`, therefor to use `println!` on it, we need to call `display()` on the value
* `String.trim_end()` removes trailign whitespaces
* `..` is in rust the syntax for range
* `..4` would mean from index `0` to index 4
* `5..` would mean from index `5` to the end
* `..` would mean from index `0` to the end

### IO

* `io::stdin()` is a function to receive user input
* `io::stdin()` returns an instance of `std::io::Stdin`, which is a type that represents a handle to the standard input
* `read_line()` is a method of an instance of `std::io::Stdin` handle
* `read_line()` receives a reference to a mutable variable to store the `string` in
* `read_line()` returns a **Result** with 2 possible variants
  * **Ok** - operation was successful
  * **Err** - operation was failed
  * with `expect()` the error case can be handled, what usually indicates a problem in the underlying OS

### Reading from files

* a `File::open()` not a string, it returns a `Result`
* a `Result` in this case, contains a `String` and an `Error`
* a `Result` is an `enum`
* to handle the enum variants, also called **arms**, we can use `match`
* `unwrap()` is the shortcut method to do the same
* `unwrap()` combines the `match` and `panic` combination
* `?` is also a for `match`
* `?` can only be used on functions that returns a `Result`
* `?` will end up in the already unwrapped result (Ok), or `Err`

### Smart-Pointer

* `Pointer` is a address to a memory location
* `Box<T>` is the most straight forward smart pointer
* a `Box<T>` smart pointer stores data on the `heap`instead of the `stack`
* `Boxes` do not have a performance overhead, but also do not have extra capabilities
* common uses cases are:
  * a type with an unknown size at compile time and a value of that type should be used
  in a context which requires an exact size
  * when a value should be owned, where it is only needed to know that
  a certain trait is implemented, rahter than a specific type

### Enumerations / enums

* is used to define a type by enumerating over its possible variants
* written as capitalized **CamelCase**
* is used to encode meaning with data
* in comparison to `structs` it is possible to group different variants, where
  each could also be an `struct` itself
* it is possible to attach data to an `enum` directly, without using a `struct`
* the data of en `enum` could be **strings**, **numeric types**, **structs** etc.
* each variant of an `enum` becomes a function to construct an instance of the `enum`
* when assigning data directly to an `enum`, it is also possible that the variants can have
  different types and associated data
* with `impl` it is possible to define `methods` on enums
* `enums` with different typed variants are better suited to pass into a function, than 
  the same amount of different `structs`
* it is possible to define functions that take in any variant of an `enum`
* `Option` is a enum for a very common scenario - the value could be something or nothing
* `Option` will help to implement the concept of  `Null`
* `Option` and the variants `Some` and `None` are always in scope
* `Option` has a generic type parameter `<T>`
* `<T>` can hold any type and is used by `Some`
* to use the `Some` value of an `Option`, it needs to be converted to `<T>`, to do
  that a common use case is to use `match` operator

### Output - Debugging & Logging

* by adding `#[derive(Debug)]` enables output for custom types
* `Debug` trait is usually output that targets **developers**
* `Display` trait is usually output that targets the **user**
* logging of output of the program should be done with `stdout` - `println!()`
* logging of errors should be done with `stderr` - `eprintln!()`
* printing to the console is slow, but there are ways to speed things up:
  1. reduce number of writes that **flush** the terminal. `println!` tells the system to **flush** the terminal everytime.
  if this is not necessary the `stdout` can be wrapped in a `BufWriter`
  2. acquire a lock on `stdout`or `stderr` and use `writeln!` to print directly
  this prevents the system from locking and unlocking the `stdout` all the time
* `dbg!` macro can be used to print to the `stderr`
* `dbg!` takes **ownership** of the expression and returns it afterwards
* `dbg!` can also receive a reference `&`

## Testing

* unit test will be in the file, where the units are which should be tested
* convention is to create a module named `tests` in each file and to annotate it with `cfg(test)`
* integration test will live externally to the `src` files in a folder called `tests`
* integration test will use the code in the same way as other code would do
* assertions are done with `assert_eq!` macro
* write integration tests for every behavior a user can encounter
* integrations test should not cover all edge cases, lean on unit tests for that
* do **not** test what you can not control

## Packages

* `PathBuf` from `std::path` is also a string, but for file system paths an works cross-platform
* `std::fs::read_to_string` will read fill to a value
* calling `.lines()` on the return value of `std::fs::read_to_string` will give me an iterable value, where each index is a line
* the result of `.lines()` is a `String`, which has a `.contains()` method, to check if a string contains a certain pattern
* `anyhow` is a package to handle errors and to have pretty printing to console
* `ansi_term` is a good package to print out raw escaped code
* `assert_cmd` adds methods on *command*
* `assert_fs` is usefule to generate temporary test files
* `predicates` assertion library
* `std:process:Command` is used to run the binary
* `rand` is used to generate random numbers
* `tabled` an easy to use library for pretty print tables of Rust `struct`s and `enum`s.

## Packaging and Distributing

* three ways to package and release a programm
* publishing with **cargo**
  * with **cargo** all external dependecies will get downloaded from **cartes.io**
  * `cargo publish` will publish the crate to **crates.io**
  * `cargo.toml` should include all necessary fields for publishing on **crates.io**
  * e.g.: name, version, authors, license, description, readme, homepage, repo, keywords, categories
  * to install a package from **crates.io** `cargo install <crate-name>` should be used
  * user of the tool need to have rust installed to compile from source and use the programm
  * should mainly be used for crates which targets other rust developers
* distributing binaries
  * `cargo build` will generate binaries
  * generates one file, that can be used by users with the same OS
  * no need for `rust` to be installed
  * no need for compilation
  * different OS needs different builds and binaries
* getting the app into a package registry
  * usually no think about how to install the tool
  * package managers provides update mechanics
  * but each OS has there own package managers
* for starters, first use `cargo install`, than binaries and at last package managers
