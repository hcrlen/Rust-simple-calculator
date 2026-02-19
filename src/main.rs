use std::{fs::read, io};

fn read_input() -> String {
    let mut input = String::new();
    let _err = io::stdin().read_line(&mut input);
    input
}

fn main() {
    println!("Enter an input to start:");
    let a = read_input();
    println!("input:{}", a);
    println!("input success!");

    println!("now input your name:");
    let name = read_input();
    println!("your name is:{}", name);

    println!("let's do some math, input two numbers:");

    let num1: i32 = read_input().trim().parse().expect("not valid");
    let num2: i32 = read_input().trim().parse().expect("not valid");
    println!("choose a function: add, subtract, multiply, divide, modulus");
    let fn_name = read_input().trim().to_lowercase();
    let res = match fn_name.as_str() {
        "add" => add(num1, num2),
        "subtract" => subtract(num1, num2),
        "multiply" => multiply(num1, num2),
        "divide" => divide(num1, num2),
        "modulus" => modulus(num1, num2),
        _ => {
            println!("invalid function name");
            return;
        }
    };

    println!(
        "the result of {}: {} and {} is: {}",
        fn_name, num1, num2, res
    );
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn divide(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        println!("cannot divide by zero");
        return 0;
    }
    a / b
}

fn modulus(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        println!("cannot modulus by zero");
        return 0;
    }
    a % b
}
