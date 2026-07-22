// Pattern Matching
// `match` is an exhaustive control flow construct.
// `if let` is concise for single-pattern matching.
// Patterns can destructure, bind, and use guards.

fn main() {
    // Basic match on integers
    let number = 3;
    match number {
        1 => println!("One"),
        2 | 3 => println!("Two or Three"),
        4..=7 => println!("Four through Seven"),
        _ => println!("Other"),
    }

    // Match on Option<T>
    let val = Some(42);
    match val {
        Some(x) if x > 40 => println!("Big value: {}", x),
        Some(x) => println!("Value: {}", x),
        None => println!("Nothing"),
    }

    // if let - concise single pattern
    let opt = Some(99);
    if let Some(v) = opt {
        println!("if let matched: {}", v);
    }

    // Destructuring a tuple
    let pair = (10, "hello");
    let (a, b) = pair;
    println!("Destructured: {} {}", a, b);

    // while let
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();

    // match with ranges and guards
    let score = 85;
    match score {
        0..=59 => println!("F"),
        60..=69 => println!("D"),
        70..=79 => println!("C"),
        80..=89 => println!("B"),
        90..=100 => println!("A"),
        _ => println!("Invalid"),
    }

    // Matching enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
    }
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
    }
}
