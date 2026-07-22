// Space Complexity
// Measures how memory usage scales with input size (n).
// Includes: input space + auxiliary space (temporary allocations).
// Common: O(1), O(n), O(n^2), O(log n) recursion stack.

fn main() {
    let n = 5;

    // O(1) - Constant: uses fixed extra space regardless of input
    let arr = vec![1, 2, 3, 4, 5];
    println!("Sum (O(1) space): {}", sum_constant(&arr));

    // O(n) - Linear: creates copy proportional to input
    let doubled = double_linear(arr.clone());
    println!("Doubled (O(n) space): {:?}", doubled);

    // O(n) recursion stack space
    println!("Factorial(5) (O(n) stack): {}", factorial_recursive(n));

    // O(1) in-place modification
    let mut data = vec![5, 3, 1, 4, 2];
    sort_in_place(&mut data);
    println!("In-place sort (O(1) space): {:?}", data);

    // O(n^2) space: matrix
    let matrix = create_matrix(3);
    println!("Matrix (O(n^2) space): {:?}", matrix);

    // Space comparison
    println!("\nSpace complexity examples:");
    println!("  O(1)   - constant:  single variable");
    println!("  O(n)   - linear:    array copy, hash map");
    println!("  O(n^2) - quadratic: 2D matrix, adjacency matrix");
    println!("  O(log n) - logarithmic: recursive calls (balanced)");
}

fn sum_constant(arr: &[i32]) -> i32 {
    let mut sum = 0; // single variable = O(1)
    for &x in arr {
        sum += x;
    }
    sum
}

fn double_linear(arr: Vec<i32>) -> Vec<i32> {
    arr.into_iter().map(|x| x * 2).collect() // new Vec = O(n)
}

fn factorial_recursive(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial_recursive(n - 1) // max n stack frames = O(n)
    }
}

fn sort_in_place(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn create_matrix(n: usize) -> Vec<Vec<usize>> {
    let mut matrix = Vec::with_capacity(n);
    for i in 0..n {
        let mut row = Vec::with_capacity(n);
        for j in 0..n {
            row.push(i * n + j);
        }
        matrix.push(row);
    }
    matrix
}
