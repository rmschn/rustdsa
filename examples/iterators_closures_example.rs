// Iterators and Closures
// Iterators are lazy; they do nothing until consumed.
// Closures are anonymous functions that can capture environment.
// Common iterator adapters: map, filter, fold, enumerate.

fn main() {
    // Closure basics
    let add_one = |x| x + 1;
    println!("Closure add_one(5) = {}", add_one(5));

    // Closure capturing environment
    let factor = 10;
    let multiply = |x| x * factor;
    println!("Closure multiply(5) = {}", multiply(5));

    // Iterator with `iter()` - borrows
    let nums = vec![1, 2, 3, 4, 5];
    let sum: i32 = nums.iter().sum();
    println!("Sum: {}", sum);

    // map - transforms each element (lazy)
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // filter - keeps elements matching predicate
    let evens: Vec<&i32> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    // chain - combine two iterators
    let more = vec![6, 7, 8];
    let combined: Vec<&i32> = nums.iter().chain(more.iter()).collect();
    println!("Chained: {:?}", combined);

    // zip - pair elements from two iterators
    let labels = vec!["a", "b", "c"];
    let zipped: Vec<(&str, &i32)> = labels.iter().copied().zip(nums.iter()).collect();
    println!("Zipped: {:?}", zipped);

    // fold (reduce)
    let product: i32 = nums.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);

    // enumerate
    for (i, val) in nums.iter().enumerate() {
        print!("[{}:{}] ", i, val);
    }
    println!();

    // Owned iterator: into_iter()
    let words = vec!["one", "two", "three"];
    for w in words {
        print!("{} ", w);
    }
    println!();

    // Mutable iterator: iter_mut()
    let mut vals = vec![1, 2, 3];
    for v in vals.iter_mut() {
        *v *= 10;
    }
    println!("After iter_mut: {:?}", vals);

    // Custom iterator adapter chain
    let result: Vec<i32> = nums
        .iter()
        .filter(|&&x| x > 2)
        .map(|&x| x * x)
        .collect();
    println!("Filter+map: {:?}", result);
}
