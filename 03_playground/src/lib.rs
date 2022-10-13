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
