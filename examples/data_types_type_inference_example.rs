// Data Types and Type Inference
// Rust is statically typed. The compiler can often infer types.
// Scalar types: integers (i8-u128), floats (f32, f64), bool, char.
// Compound types: tuples, arrays.

fn main() {
    // Type inference - compiler deduces the type
    let guess = "42".parse::<i32>().unwrap();
    println!("Parsed guess: {}", guess);

    // Integer types
    let a: i8 = -128;
    let b: u8 = 255;
    let c: i32 = -2_147_483_648;
    let d: u64 = 18_446_744_073_709_551_615;
    println!("i8: {}, u8: {}, i32: {}, u64: {}", a, b, c, d);

    // Float types
    let e: f32 = 3.14;
    let f: f64 = 2.718_281_828;
    println!("f32: {}, f64: {}", e, f);

    // Boolean
    let g: bool = true;
    let h = false;
    println!("bool true: {}, false: {}", g, h);

    // Character
    let c1: char = 'Z';
    let c2 = '😊';
    println!("char: {}, emoji: {}", c1, c2);

    // Tuple
    let tup: (i32, f64, char) = (42, 3.14, 'R');
    let (x, y, z) = tup; // destructuring
    println!("Tuple: ({}, {}, {}), access by index: {}", x, y, z, tup.0);

    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let same = [0; 3]; // [0, 0, 0]
    println!("Array: {:?}, same: {:?}", arr, same);
}
