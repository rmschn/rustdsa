// Slices and References
// A slice is a reference to a contiguous sequence of elements in a collection.
// References (&) borrow a value without taking ownership.
// String slices: &str, array slices: &[T].

fn main() {
    // String slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("String slices: '{}' '{}'", hello, world);

    // Whole string slice
    let whole = &s[..];
    println!("Whole slice: '{}'", whole);

    // Array slices
    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[1..4];
    println!("Array slice: {:?}", slice);

    // Mutable array slice
    let mut nums = [1, 2, 3, 4, 5];
    let mid = &mut nums[1..4];
    mid[0] = 99;
    println!("After mut slice: {:?}", nums);

    // Function taking slice
    let words = String::from("one two three");
    let first = first_word(&words);
    println!("First word: '{}'", first);

    // Reference basics
    let val = 42;
    let ref_val = &val;
    println!("Value: {}, ref: {}, deref: {}", val, ref_val, *ref_val);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
