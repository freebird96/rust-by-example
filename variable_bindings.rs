use std::convert::From;
use std::fmt;

#[derive(Debug)]
struct Number {
    value: u32,
}

struct Circle {
    radius: u32
}


impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius: {:?}", self.radius)
    }
}


impl From<u32> for Number {
    fn from(item: u32) -> Self {
        Number {
            value: item
        }
    }
}

fn main() {
    let num = Number::from(40);
    println!("My number is {:?}", num);
    println!("The Number value is {:?}", num.value);
    let circle = Circle {radius: 6};
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed: i32 = "42.99".parse::<i32>().unwrap();
    println!("The sum is: {}", parsed+turbo_parsed)
}