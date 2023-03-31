// use std::{convert::From};

// #[allow(dead_code)]
// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// impl From<i32> for Number {
//     fn from(item: i32) -> Self {
//         Number { value: item }
//     }
// }

// fn basic_fn() {
//     println!("This is just a basic function with no arguments.");
// }

// fn fn_with_inputs(num1: i32, num2: i32) {
//     println!("The num1={} and num2={}", num1, num2);
// }

// fn fn_with_inputs_and_outputs(num1: i32, num2: i32) -> i32 {
//     num1 * num2
// }

// fn main() {
//     basic_fn();
//     fn_with_inputs(23, 32);
//     let product = fn_with_inputs_and_outputs(23, 32);
//     println!("The product of the two numbers are = {}", product);
//     let full_name = {
//         let first_name = "Noble";
//         let last_name = "Varghese";
//         format!("{} {}", first_name, last_name)
//     };
//     println!("The full name is {}", full_name);
//     let mut n = String::new();
//     std::io::stdin()
//         .read_line(&mut n)
//         .expect("Failed to read the input");
//     let n: f64 = n.trim().parse().expect("Invalid input. Input not a float value");
//     println!("The input from the user is {}", n)

// }

/*
   Rust Ownership
       - Each value has a variable and that's called its owner
       - There can be only one owner at a time.
       - When the owner goes out of scope, the value will be dropped.

    Stack overflow might happen when variables are continuously declared on the stack in a loop.
    Why is ownship moved or transfered in case of non-primitive type such as a string or a vector.
        This is because the memory allocation for those types happen on the heap memory and not on the stack. Using the heap involves
        the use of the OS for memory allocation and returning a pointer to that location.
*/

// fn main() {
//     let x = 32.6;
//     let y = x;
//     println!("x: {} and y:{}", x, y);

//     let s1 = String::from("abc");
//     let s2 = & s1;
//     println!("the values of s1={} and s2={}", s1, s2);
// }

// use std::vec;

// fn stack_func(mut num: i64) {
//     println!("The current value of the number is {}", num);
//     num = 67;
//     println!("The updated number is {}", num);
// }

// fn heap_func(var: &mut Vec<i32>) {
//     println!("The first value of var is {:?}", var);
//     var.push(60);
//     println!("Var: {:?}", var);
// }

// fn main() {
//     let stack_num = 32;
//     let mut heap_vec = vec![4, 5, 6];
//     stack_func(stack_num);
//     println!(" The value inside the main of stack_num is {}", stack_num);
//     heap_func(&mut heap_vec);
//     println!(" The value inside the main of heap_num is {:?}", heap_vec);

//     let string1 = String::from("This is a long string");
//     let string2 = String::from("This is another long string");

//     // Here we have combined the two strings without copying the contents of the two strings.
//     let vec_string = vec![&string1, &string2];
//     println!("The value of the combined strings are: {:?}", vec_string);
// }

// fn main() {
//     // let mut heap_num = vec![4, 5, 6];
//     // let ref1 = &mut heap_num;
//     // let ref2 = &mut heap_num;
//     // println!("ref1: {:?} and ref2", ref1);

//     let mut heap_num = vec![4, 5, 6];
//     let ref2 = &heap_num;
//     let ref3 = &heap_num;
//     let ref1 = &mut heap_num;
//     println!(
//         "ref1: {:?} and ref2: {:?} and heap: {:?}",
//         ref1, ref2, heap_num
//     );
// }

// use std::io::Read;

// fn some_fn(a1: &String, a2: &str) {
//     println!("The string values are {} & {}", a1, a2)
// }

// fn main() {
//     let s1 = String::from("String value from 1");
//     let s2 = "myself another string";
//     some_fn(&s1, s2);

//     println!("The existing ones are {} {}", s1, s2);
//     let marks = 65;
//     if marks <= 70 && marks >= 60 {
//         println!("the stiudent has a distinction.!");
//     }

//     // println!("Guess any number between 1 and 20.");
//     // while guess != true {
//     //     let mut number = String::new();
//     //     std::io::stdin()
//     //         .read_line(&mut number)
//     //         .expect("Failed to read the input");

//     //     let number = number.trim().parse().expect("Invalid input type");
//     //     if my_number == number {
//     //         println!("You guessed the number correctly !");
//     //         guess = true;
//     //     } else {
//     //         println!("Please try again !!");
//     //     }
//     // }

//     // println!("Enter any number and I can tell you the next number that is divisible by 2 and 5");
//     // let mut number = String::new();
//     // std::io::stdin()
//     //     .read_line(&mut number)
//     //     .expect("Failed to read the input");
//     // let mut number: i32 = number.trim().parse().expect("Invlaid input type");
//     // let mut divisible_by_2_5 = false;
//     // while divisible_by_2_5 != true {
//     //     number += 1;
//     //     if number % 2 == 0 && number % 5 == 0 {
//     //         println!("Number divisible by 2 and 5 is {}", number);
//     //         divisible_by_2_5 = true;
//     //     }
//     // }

//     let mut some_vec = vec![20, 30, 40, 50, 60, 70];
//     for i in &mut some_vec {
//         *i += 5;
//         println!("{:?}", i);
//     }
//     println!("{:?}", some_vec);
// }

/*
    Write a program for finding the difference of the square of the sum and the sum of square of the first N number (where N is a user defined input that you program will take). for instance, if the user enters the number of let say 5,
Then you should first compute the squae of sum = (1+2+3+4+5)^2  = 225

and next you will compute the sum of square as  = (1^2  + 2^2  + 3^2  + 4^2  + 5^2)  = (1 + 4+ 9 + 16 +25 ) = 55

and finally you will compute the difference = 225 - 55 = 170.

 */

// fn compute_sq(num: i32) -> i32 {
//     num * num
// }

// fn compute_sq_sum(num: i32) -> i32 {
//     let mut sum_sq = 0;
//     for i in 0..=num {
//         sum_sq += compute_sq(i);
//     }
//     sum_sq
// }

// fn compute_sum_sq(num: i32) -> i32 {
//     let mut sum = 0;
//     for i in 0..=num {
//         sum += i
//     }
//     sum * sum
// }

// fn main() {
//     println!("enter a number");
//     let mut input = String::new();
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read the input");
//     let input: i32 = input.trim().parse().expect("Invalid type");
//     let difference = compute_sum_sq(input) - compute_sq_sum(input);
//     println!("The answer is {}", difference);
// }

// fn main() {
//     println!("enter a number");
//     let mut input = String::new();
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read the input");
//     let input: i32 = input.trim().parse().expect("Invalid type");
//     let mut sum = 0;
//     for i in 0..input {
//         if i % 3 == 0 && i % 5 == 0 {
//             println!("{}", i);
//             sum += i;
//         } else if i % 3 == 0 {
//             println!("{}", i);
//             sum += i;
//         } else if i % 5 == 0 {
//             println!("{}", i);
//             sum += i;
//         }
//     }
//     println!("The sum is {}", sum);
// }

fn new_stack(max_size: usize) -> Vec<u32> {
    let vec = Vec::with_capacity(max_size);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let popped_val = stack.pop();
    println!("The popped value is {:?}", popped_val as Option<u32>);
    popped_val
}

fn push(num: u32, max_size: usize, stack: &mut Vec<u32>) {
    if stack.len() == max_size {
        println!("Stack is full...");
    } else {
        stack.push(num);
        println!("The stack is {:?}", stack);
    }
}

fn input() -> u32 {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Value not found.");
    let n: u32 = n.trim().parse().expect("Invalid input");
    n
}

fn main() {
    println!("Please mention the size of the stack");
    let stack_size = input();
    let mut new_stack = new_stack(stack_size as usize);
    loop {
        println!("Choose one....");
        println!("1. Push \n2. Pop \n3. Display \n4.Size \n5.Exit");
        let choice = input();
        match choice {
            1 => {
                println!("Enter the input to apppend to the stack.");
                let ele = input();
                push(ele, stack_size as usize, &mut new_stack);
            }
            2 => {
                pop(&mut new_stack);
            }
            3 => {
                println!("the current stack is {:?}", new_stack);
            }
            4 => {
                println!("The size of the current stack is {}", new_stack.len());
            }
            5 => break,
            _ => println!("Exiting..."),
        }
        println!("Choose 1 to continue and 0 to exit");
        let break_choice = input();
        println!("Break choice = {}", break_choice);
        if break_choice == 1 {
            continue;
        } else {
            println!("Ending the program.");
            break;
        }
    }
}
