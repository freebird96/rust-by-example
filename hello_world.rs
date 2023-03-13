// Structures

#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

struct Rectangle {
    length: u32,
    breadth: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.breadth
}

fn main() {
    // let name = String::from("Peter");
    // let age =  28;
    // let peter = Person {name, age};

    let rectangle = Rectangle {
        length: 30,
        breadth: 50,
    };
    println!("The area of the rectangle is {:?}", area(&rectangle))
}