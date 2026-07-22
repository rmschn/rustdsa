// Sets
// Unordered collection of unique elements. O(1) average lookup.
// Rust's HashSet uses the same SipHash as HashMap.
// Operations: insert, remove, contains, union, intersection, difference.

use std::collections::HashSet;

fn main() {
    // Create
    let mut books = HashSet::new();
    books.insert("1984");
    books.insert("Brave New World");
    books.insert("Fahrenheit 451");
    println!("Books: {:?}", books);

    // Insert (duplicates ignored)
    books.insert("1984"); // no effect
    println!("Len: {}", books.len());

    // Contains
    println!("Contains '1984': {}", books.contains("1984"));
    println!("Contains 'Dune': {}", books.contains("Dune"));

    // Remove
    books.remove("Brave New World");
    println!("After remove: {:?}", books);

    // From iterator
    let nums: HashSet<i32> = vec![1, 2, 3, 4, 5, 5, 5].into_iter().collect();
    println!("Unique nums: {:?}", nums);
    println!("Len: {} (duplicates removed)", nums.len());

    // Set operations
    let a: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();

    // Union
    let union: HashSet<_> = a.union(&b).cloned().collect();
    println!("Union: {:?}", union);

    // Intersection
    let intersection: HashSet<_> = a.intersection(&b).cloned().collect();
    println!("Intersection: {:?}", intersection);

    // Difference (in a but not b)
    let diff: HashSet<_> = a.difference(&b).cloned().collect();
    println!("A - B: {:?}", diff);

    // Symmetric difference
    let sym_diff: HashSet<_> = a.symmetric_difference(&b).cloned().collect();
    println!("Symmetric diff: {:?}", sym_diff);

    // Subset / superset
    let sub: HashSet<i32> = vec![1, 2].into_iter().collect();
    println!("Subset? {}", sub.is_subset(&a));
    println!("Superset? {}", a.is_superset(&sub));
}
