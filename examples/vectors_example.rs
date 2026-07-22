// Vectors
// Growable array (heap-allocated). Type: Vec<T>.
// Provides O(1) amortized push, O(1) index access.

fn main() {
    // Create via vec! macro
    let mut v1 = vec![1, 2, 3];
    println!("Initial: {:?}", v1);

    // Push (add to end)
    v1.push(4);
    v1.push(5);
    println!("After push: {:?}", v1);

    // Pop (remove from end)
    let last = v1.pop();
    println!("Popped: {:?}, vec: {:?}", last, v1);

    // Index access
    println!("v[0] = {}, v[2] = {}", v1[0], v1[2]);

    // Safe get (returns Option)
    match v1.get(10) {
        Some(val) => println!("v[10] = {}", val),
        None => println!("v[10] out of bounds"),
    }

    // Insert and remove
    v1.insert(1, 99); // insert at index 1
    println!("After insert 99 at 1: {:?}", v1);
    let removed = v1.remove(2);
    println!("Removed v[2] = {}, vec: {:?}", removed, v1);

    // Iterate
    print!("Iterate: ");
    for val in &v1 {
        print!("{} ", val);
    }
    println!();

    // Mutable iteration
    for val in &mut v1 {
        *val *= 2;
    }
    println!("Doubled: {:?}", v1);

    // Vector with capacity
    let mut v2: Vec<i32> = Vec::with_capacity(100);
    println!("Capacity: {}, len: {}", v2.capacity(), v2.len());
    v2.push(10);
    println!("After push - Capacity: {}, len: {}", v2.capacity(), v2.len());

    // Collect from iterator
    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("Squares: {:?}", squares);

    // Vector of enums (heterogeneous data)
    #[derive(Debug)]
    enum Value {
        Int(i32),
        Text(String),
    }
    let mixed = vec![
        Value::Int(42),
        Value::Text(String::from("hello")),
        Value::Int(99),
    ];
    println!("Mixed vec: {:?}", mixed);

    // Sorting
    let mut unsorted = vec![5, 3, 1, 4, 2];
    unsorted.sort();
    println!("Sorted: {:?}", unsorted);
}
