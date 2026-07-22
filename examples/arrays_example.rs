// Arrays
// Fixed-size, homogeneous collection. Stored on stack.
// Type signature: [T; N] where T is element type, N is compile-time constant length.
// Bounds-checked at runtime.

fn main() {
    // Explicit type and length
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr1: {:?}", arr1);

    // Initialize all elements to same value
    let arr2 = [0; 4]; // [0, 0, 0, 0]
    println!("arr2: {:?}", arr2);

    // Access elements
    println!("First: {}, Third: {}", arr1[0], arr1[2]);

    // Length
    println!("Length: {}", arr1.len());

    // Iterate
    print!("Iterate: ");
    for elem in arr1 {
        print!("{} ", elem);
    }
    println!();

    // Iterate with index
    for (i, val) in arr1.iter().enumerate() {
        print!("[{}:{}] ", i, val);
    }
    println!();

    // Array slice
    let slice = &arr1[1..4];
    println!("Slice [1..4]: {:?}", slice);

    // Array as stack-allocated, fixed size
    // vs Vec which is heap-allocated, growable
    let mut arr3 = [10, 20, 30];
    arr3[1] = 99;
    println!("Mutable array: {:?}", arr3);

    // Multi-dimensional arrays
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("Matrix[1][2] = {}", matrix[1][2]);

    // Array patterns
    let [a, b, c] = [1, 2, 3];
    println!("Destructured: {}, {}, {}", a, b, c);
}
