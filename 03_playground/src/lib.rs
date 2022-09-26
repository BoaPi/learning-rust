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
