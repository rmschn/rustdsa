// Ownership and Borrowing
// Ownership rules:
//   1. Each value has one owner.
//   2. When the owner goes out of scope, the value is dropped.
//   3. Ownership can be transferred (move) or borrowed (&, &mut).
// Borrowing: reference without taking ownership.
// Mutable borrow: one at a time. No other references during a mutable borrow.

fn main() {
    // Ownership: s1 owns the String
    let s1 = String::from("hello");
    let s2 = s1; // move: s1 is invalidated, s2 owns it
    // println!("{}", s1); // ERROR: s1 moved
    println!("After move, s2 = {}", s2);

    // Clone (deep copy)
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // Borrowing (immutable reference)
    let s5 = String::from("borrow");
    let len = calculate_length(&s5); // &s5 borrows s5
    println!("Length of '{}' is {}", s5, len); // s5 still usable

    // Mutable borrow
    let mut s6 = String::from("mut");
    change(&mut s6);
    println!("After mutable borrow: {}", s6);

    // Scope of references
    let mut s7 = String::from("scope");
    let r1 = &s7;     // imm borrow
    let r2 = &s7;     // imm borrow OK (many)
    println!("{} and {}", r1, r2);
    // r1 and r2 no longer used here
    let r3 = &mut s7; // mut borrow OK
    r3.push_str("!");
    println!("After mut borrow: {}", r3);

    // Dangling reference prevented by compiler (won't compile)
    // let r = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("ated");
}
