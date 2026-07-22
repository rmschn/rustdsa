// Strings
// Two string types: &str (string slice, borrowed) and String (owned, growable).
// String is UTF-8 encoded. Not indexable by byte position directly.

fn main() {
    // String literal (&str)
    let greeting: &str = "Hello";
    println!("Literal: {}", greeting);

    // String (owned)
    let mut s = String::from("Hello");
    s.push_str(", world!"); // append &str
    s.push('!'); // append char
    println!("String: {}", s);

    // Concatenation
    let s1 = String::from("Hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // s1 moved, s2 borrowed
    println!("Concatenated: {}", s3);

    // Format macro (doesn't take ownership)
    let a = String::from("foo");
    let b = String::from("bar");
    let c = format!("{}-{}", a, b);
    println!("Formatted: {}", c);
    println!("Originals still usable: {}, {}", a, b);

    // String slicing (by byte, careful with UTF-8)
    let hello = "Здравствуйте";
    let slice = &hello[0..4]; // "Зд" (each char is 2 bytes)
    println!("UTF-8 slice [0..4]: '{}'", slice);

    // Iterate over chars
    print!("Chars: ");
    for c in "Hello".chars() {
        print!("{} ", c);
    }
    println!();

    // Iterate over bytes
    print!("Bytes: ");
    for b in "Hi".bytes() {
        print!("{} ", b);
    }
    println!();

    // String methods
    let text = String::from("  Rust Programming  ");
    println!("Trimmed: '{}'", text.trim());
    println!("Uppercase: '{}'", text.to_uppercase());
    println!("Replace: '{}'", text.replace("Rust", "C++"));
    println!("Contains 'Prog': {}", text.contains("Prog"));

    // Split
    let words: Vec<&str> = "one,two,three".split(',').collect();
    println!("Split: {:?}", words);

    // to_string() on &str
    let owned = "literal".to_string();
    println!("to_string: {}", owned);

    // &str from String (deref coercion)
    let os = String::from("deref");
    let slice: &str = &os;
    println!("Deref to &str: {}", slice);
}
