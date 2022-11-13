// custom enum lesson
// ip address enum examples
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// custom method for enum IpAddr
impl IpAddr {
    fn call(&self) {
        println!("{:?}", self)
    }
}

pub fn custom_enum_lesson() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    home.call();
    loopback.call();
}

// immutability/ mutability lesson
pub fn immutability_lesson() {
    // next line wont compile, because of immutability
    // let x: i32 = 5;
    let mut x: i32 = 5;
    println!("{}", x);

    x = 6;
    println!("{}", x);
}

// how to shadow a variable
pub fn shadowing_lesson() {
    // first declaration
    let x: i32 = 5;

    // first shadowing
    let x = x + 1;

    // shadowing is also scope related
    {
        let x = x * 2;
        println!("The value of x is {}, in this scope", x);
    }

    // next println!() is not effected by the scoped shadoing
    println!("The value of x is {}, uneffected by the former scope", x);
}

pub fn simple_data_type_lesson() {
    // type needs to be annotated properly
    // otherwise the compile is not able to know the type at compile time
    // let guess = "42".parse().expect("Not an integer");
    let guess: i32 = "42".parse().expect("Not an integer");
    println!("Your guess is: {}", guess);
}

pub fn basic_numeric_operations_lesson() {
    // addition
    let sum: u32 = 1 + 5;
    println!("Sum is: {}", sum);

    // subtraction
    let difference: f64 = 10.1 - 5.05;
    println!("Difference is: {}", difference);

    // multiplication
    let product: i32 = 5 * 12;
    println!("Product is: {}", product);

    // division
    let div1: f64 = 125.3 / 40.1;
    println!("Quotient 1 is: {}", div1);

    let div2: i32 = 2 / 3;
    println!("Quotient 2 is: {}", div2);

    // remainder
    let remainder1: u8 = 53 % 7;
    println!("Remainder 1 is: {}", remainder1);

    let remainder2: f64 = 53.5 % 3.4;
    println!("Remainder 2 is {}", remainder2);
}

pub fn basic_boolean_lesson() {
    // type inherit
    let t = true;

    // explicite anotation
    let f: bool = false;

    println!("t is {:?} - f is {:?}", t, f);
}

pub fn basic_character_lesson() {
    // type inherit
    let c = 'z';

    let z: char = 'Z';

    let party_emoji: char = '🥳';

    println!(
        "Some chars are lowercase: {}, some uppercase: {} and other are just party: {}",
        c, z, party_emoji
    );
}

pub fn basic_tuple_lesson() {
    let tup: (i32, u16, u8) = (-100, 50, 255);

    // destructuring from a tuple
    let (x, y, z) = tup;

    println!("Tuple {:?}", tup);
    println!("Tuple x {}, y {}, z {}", x, y, z);

    // period access
    let period_access = tup.2;
    println!("Tuple element #3: {}", period_access);

    // unit case of a tuple
    let unit_case: () = ();
    println!("Unit tuple: {:?}", unit_case);
}

pub fn basic_array_lesson() {
    // simple integer array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Integer array {:?}", a);

    // simple string array
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Months array {:?}", months);
}

pub fn extended_array_lesson() {
    // define array
    let a: [i8; 5] = [1, 2, 3, 4, 5];

    println!("Array entry 4 is {}", a[4]);
}

fn example_function(x: i16) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i16, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn implicit_retunr() -> char {
    let test: char = 't';
    test
}

pub fn basic_function_lesson() {
    // use function
    example_function(5);
    print_labeled_measurement(8, 'm');
    println!("implicit return {}", implicit_retunr());
}

pub fn basic_loop_lesson() {
    // set counter
    let mut counter: i8 = 0;

    // setup loop
    let result = loop {
        println!("Interim result {}", counter);

        if counter == 5 {
            println!("Interim result {}", counter);
            counter = counter * 2;
            continue;
        }

        if counter == 10 {
            println!("Interim result {}", counter);
            break counter * 2;
        }

        counter += 1;
    };

    println!("The result is {}", result);
}

pub fn nesting_loops_lesson() {
    // setup counter
    let mut counter: u8 = 0;

    // labeling outter loop
    'counting_up: loop {
        println!("count {counter}");

        // setup remaing loops
        let mut remaining: u8 = 20;

        loop {
            println!("remaining {remaining}");

            if remaining == 8 {
                break;
            }

            if counter == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        counter += 1;
    }

    println!("Endcounter {counter}");
}

pub fn while_loop_lesson() {
    // setting up the condition
    let mut number: u8 = 10;

    while number != 0 {
        println!("Number value is {number}");

        number -= 1;
    }

    println!("LIFT OF!!!");
}

pub fn for_loop_lesson() {
    // create an collection to loop though
    let collection: [char; 6] = ['a', 'r', 'r', 'a', 'y', 's'];

    for c in collection {
        println!("This is {c}");
    }
}

pub fn while_to_for_loop_lesson() {
    // converting the while_loop_lesson to use for loop instead
    for number in (1..11).rev() {
        println!("Countdown in {number}");
    }

    println!("LIFT OF!!!");
}

pub fn fibonacci_lesson(nth: usize) {
    // setup the first two numbers to start
    let mut former: usize = 0;
    let mut current: usize = 1;
    let mut counter: usize = 2;
    let mut interim: usize;

    if nth == 0 {
        println!("There is no {nth} of the fibonacci");
    } else if nth == 1 {
        println!("The {nth} of the fibonacci is {former}");
    } else if nth == 2 {
        println!("The {nth} of the fibonacci is {current}");
    } else {
        while counter < nth {
            // add former and current to new former and current
            interim = former + current;
            former = current;
            current = interim;

            // increase counter
            counter += 1;
        }

        // when loop is donw print out desired number
        println!("The {nth} of the fibonacci is {current}");
    }
}

#[derive(PartialEq)]
pub enum Scale {
    Celsius,
    Fahrenheit,
}

impl Scale {
    pub fn convert(self, t: f64) {
        // setup result
        let result: f64;
        if self == Scale::Celsius {
            result = (t * 9.0 / 5.0) + 32.0;
            println!("{t} celsius in fahrenheit is {result}");
        } else {
            result = (t - 32.0) * 5.0 / 9.0;
            println!("{t} fahrenheit in celsius is {result}");
        }
    }
}

pub fn basic_string_lesson() {
    // setup mutable string literal
    let mut s = String::from("can not be mutated");

    // if we would define s as shown in the next line, the program won`t compile
    // let mut s = "can not be mutated");

    s.push_str(", or?");

    println!("{}\nwas mutated", s);
}

pub fn ownership_lesson() {
    //======================================
    // copy value over into other variable
    // both live now in the stack
    let x: i32 = 5;
    let y: i32 = x;

    println!("x is {x} and y is {y}");

    //======================================
    // String type are differnet only parts will be in the stack
    // the actual content lives i the heap
    {
        let s1: String = String::from("Hello World");
        // s1 is no longer valid, because ownership changed to s2
        // s1 is already dropped here
        let s2: String = s1;

        println!("s1 was {s2} and s2 is {s2}");
    }

    //======================================
    // to use both strings after assigning to another variable
    // "clone" can be used
    {
        let s1: String = String::from("Hello again");
        let s2: String = s1.clone();

        println!("s1 is still {s1} and s2 is {s2} as arbitrary copy");
    }

    // function for demonstration
    fn takes_ownership(some_string: String) {
        println!("some string is: {some_string}");
    }

    fn makes_copy(some_integer: i32) {
        println!("some integer is: {some_integer}");
    }

    fn gives_ownrship() -> String {
        let inner_s: String = String::from("your ownership");

        inner_s
    }

    fn takes_and_gives_back_ownership(mut some_string: String) -> (String, usize) {
        some_string.push_str(" - MODIFIED");
        let lenght: usize = some_string.len();

        (some_string, lenght)
    }

    fn takes_in_a_reference(s: &String) -> usize {
        s.len()
    }

    //======================================
    // s will go out of scope and cant not be used after
    // moved to function
    {
        let s: String = String::from("I'm going to be moved");

        takes_ownership(s);

        // following line won't compile, due to s got moved already
        // and is not valid anymore
        // println!("s is no longer valid here: {s}");
    }

    //======================================
    // i will be copied and can be used after passed into function
    {
        let i: i32 = 5;

        makes_copy(i);

        println!("I'm still valid and in scope as i: {i}");
    }

    //======================================
    // s1 will granted ownership
    // s2 ownership is moved to function and s3 will get ownership
    // of the returned value
    {
        let s1: String = gives_ownrship();
        let s2: String = String::from("Ownership");
        let (s3, i) = takes_and_gives_back_ownership(s2);

        println!("s1 is: {s1}");
        // the next line is invalid, because s2 was moved
        // println!("s2 is: {s2}");
        println!("s3 is: {s3} and of lenght {i}");
    }

    //======================================
    // s1 will granted ownership
    // s1 value will only be used as a reference
    // len will get ownership of the returned value
    {
        let s1: String = gives_ownrship();

        let len = takes_in_a_reference(&s1);
        println!("len is {len}");
    }

    //======================================
    // change function will receive a mutable reference
    // s1 will be mutated after passed to change function
    {
        fn change(some_string: &mut String) {
            some_string.push_str(" Mutated Reference");
        }

        let mut s1: String = gives_ownrship();
        println!("s1 before mutate: {s1}");

        change(&mut s1);
        println!("s1 after mutate: {s1}");
    }

    //======================================
    // show case multiple references
    // and only one mutable reference is allowed after the
    // first references are out of scope
    {
        fn change(some_string: &mut String) {
            some_string.push_str(" Mutated Reference");
        }

        let mut s1: String = gives_ownrship();
        let r1: &String = &s1;
        let r2: &String = &s1;

        println!("double reference of {s1} - r1 {r1} & r2 {r2}");

        change(&mut s1);
        println!("s1 after mutate: {s1}");
    }
}

pub fn first_slice_lesson() {
    // goal is to find the first word in the given string
    // the problem with this function is, that the returned index
    // is unrelated to the string
    // if the string gets mutated or deleted, "example" would
    // still hold a usize number, which has no meaning
    fn first_word(s: &String) -> usize {
        // convert string to byte array to look for white spaces
        let bytes = s.as_bytes();

        // take byte array, covert to iterator with tuple
        // of index & reference of the value
        // because the it is a reference to an value, we use "&"
        for (i, &item) in bytes.iter().enumerate() {
            // b'' is byte literal which checks for spaces
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let example = String::from("Example String with five words");
    let result = first_word(&example);

    println!("Index of the first space is {result}");
}

pub fn second_slice_lesson() {
    // same goal as first_word() function
    // takes in a reference to a string slice
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }

    let example = String::from("Example String with five words");
    let result = first_word(&example);

    println!("First word of the string is: {result}");

    // different use cases
    let my_string = String::from("hello world!");

    // function takes in a reference to a string slice
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);

    // functions takes in a reference to a string
    let _word = first_word(&my_string);

    // also works with string literals
    let my_string_literal = "Hello World!";

    // function takes in a reference to a string literal slice
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    // functions takes in a reference to a string literal
    let word = first_word(&my_string_literal);

    println!("Slice Lesson done. Word is: {word}")
}
