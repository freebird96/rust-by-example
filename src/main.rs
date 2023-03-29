use std::{convert::From};

#[allow(dead_code)]
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn basic_fn() {
    println!("This is just a basic function with no arguments.");
}

fn fn_with_inputs(num1: i32, num2: i32) {
    println!("The num1={} and num2={}", num1, num2);
}

fn fn_with_inputs_and_outputs(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn main() {
    basic_fn();
    fn_with_inputs(23, 32);
    let product = fn_with_inputs_and_outputs(23, 32);
    println!("The product of the two numbers are = {}", product);
    let full_name = {
        let first_name = "Noble";
        let last_name = "Varghese";
        format!("{} {}", first_name, last_name)
    };
    println!("The full name is {}", full_name);
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the input");
    let n: f64 = n.trim().parse().expect("Invalid input. Input not a float value");
    println!("The input from the user is {}", n)

}

// BAsic function
/*
   Basic function
   Fn with inputs
   Fn with inputs and outputs
   Fn with multiple inputs and outputs
*/
