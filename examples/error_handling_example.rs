// Error Handling
// Rust has two error categories: recoverable (Result<T, E>) and unrecoverable (panic!).
// `?` operator propagates errors. `unwrap`/`expect` extract or panic.

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn main() {
    // panic! - unrecoverable
    // panic!("crash and burn"); // uncomment to see

    // Result enum - recoverable
    let f = File::open("nonexistent.txt");
    match f {
        Ok(file) => println!("File opened: {:?}", file),
        Err(e) => println!("Error opening file: {}", e),
    }

    // unwrap - panics on Err
    // let f = File::open("missing.txt").unwrap(); // would panic

    // expect - panics with custom message
    // let f = File::open("missing.txt").expect("File not found!");

    // Using ? operator (in a function returning Result)
    match parse_file() {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }

    // Combining with Option
    let nums = vec!["10", "20", "invalid", "30"];
    for s in nums {
        match parse_number(s) {
            Ok(n) => print!("{} ", n),
            Err(e) => print!("err:{} ", e),
        }
    }
    println!();

    // Custom error type
    match divide(10, 0) {
        Ok(v) => println!("10/0 = {}", v),
        Err(MathError::DivisionByZero) => println!("Cannot divide by zero!"),
    }
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // ? propagates error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_file() -> Result<i32, ParseIntError> {
    let content = read_file("/tmp/test.txt").unwrap_or_default();
    content.trim().parse::<i32>()
}

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

enum MathError {
    DivisionByZero,
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}
