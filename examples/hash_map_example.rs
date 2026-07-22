// Hash Map
// Key-value store with O(1) average lookup/insert/delete.
// Uses a hash function to map keys to buckets.
// Rust's HashMap uses SipHash for DoS resistance.

use std::collections::HashMap;

fn main() {
    // Create
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 87);
    scores.insert(String::from("Charlie"), 92);
    println!("Scores: {:?}", scores);

    // Access
    let name = String::from("Bob");
    match scores.get(&name) {
        Some(score) => println!("{}'s score: {}", name, score),
        None => println!("{} not found", name),
    }

    // Update
    scores.entry(String::from("Alice")).or_insert(100);
    scores.entry(String::from("David")).or_insert(88);
    println!("After entry API: {:?}", scores);

    // Overwrite
    scores.insert(String::from("Alice"), 99);
    println!("After overwrite: {:?}", scores);

    // Iterate
    for (name, score) in &scores {
        println!("  {}: {}", name, score);
    }

    // From iterator of pairs
    let pairs = vec![("x", 10), ("y", 20), ("z", 30)];
    let map: HashMap<_, _> = pairs.into_iter().collect();
    println!("From iterator: {:?}", map);

    // Word count
    let text = "hello world hello rust hello everyone";
    let mut word_count: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }
    println!("Word count: {:?}", word_count);

    // Check existence
    println!("Contains 'hello': {}", word_count.contains_key("hello"));
    println!("Contains 'foo': {}", word_count.contains_key("foo"));

    // Length
    println!("Number of entries: {}", word_count.len());
}
