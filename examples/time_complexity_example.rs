// Time Complexity
// Measures how runtime scales with input size (n).
// Big O notation: upper bound of growth rate.
// Common complexities: O(1), O(log n), O(n), O(n log n), O(n^2), O(2^n).

use std::time::Instant;

fn main() {
    let n = 1000;

    // O(1) - Constant: array access, arithmetic
    println!("O(1) - Constant: {}", constant_time(n));

    // O(log n) - Logarithmic: binary search
    let data: Vec<i32> = (0..n as i32).collect();
    let start = Instant::now();
    let _ = data.binary_search(&(n as i32 - 1));
    println!("O(log n) - Binary search: {:?}", start.elapsed());

    // O(n) - Linear: single loop
    let start = Instant::now();
    linear_time(n);
    println!("O(n) - Linear: {:?}", start.elapsed());

    // O(n log n) - Linearithmic: merge sort, heap sort
    let mut rng_data: Vec<i32> = (0..n as i32).rev().collect();
    let start = Instant::now();
    rng_data.sort();
    println!("O(n log n) - Sort: {:?}", start.elapsed());

    // O(n^2) - Quadratic: nested loops (bubble sort on unsorted)
    let start = Instant::now();
    quadratic_time(100); // keep n small
    println!("O(n^2) - Quadratic (n=100): {:?}", start.elapsed());

    // O(2^n) - Exponential: naive fibonacci
    let start = Instant::now();
    println!("O(2^n) - fib(30): {}", fib_exponential(30));
    println!("O(2^n) - Exponential (fib 30): {:?}", start.elapsed());
}

fn constant_time(_n: usize) -> &'static str {
    "Always takes same time regardless of input"
}

fn linear_time(n: usize) -> usize {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }
    sum
}

fn quadratic_time(n: usize) {
    let mut _sum = 0;
    for i in 0..n {
        for j in 0..n {
            _sum += i + j;
        }
    }
}

fn fib_exponential(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_exponential(n - 1) + fib_exponential(n - 2),
    }
}
