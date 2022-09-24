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

pub fn simple_data_type() {
    // type needs to be annotated properly
    // otherwise the compile is not able to know the type at compile time
    let guess: i32 = "42".parse().expect("Not an integer");
    print!("Your guess is: {}", guess);
}
