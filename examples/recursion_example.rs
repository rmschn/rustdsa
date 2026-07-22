// Recursion
// A function that calls itself. Requires a base case to terminate.
// Common examples: factorial, fibonacci, binary search, tree traversals.
// Each recursive call uses stack space.

fn main() {
    // Factorial
    println!("Factorial(5) = {}", factorial(5));
    println!("Factorial(0) = {}", factorial(0));

    // Fibonacci
    println!("Fibonacci(10) = {}", fibonacci(10));

    // Sum of natural numbers
    println!("Sum(1..100) = {}", sum_to(100));

    // Power (exponentiation)
    println!("2^10 = {}", power(2, 10));

    // GCD (Euclidean algorithm)
    println!("GCD(48, 18) = {}", gcd(48, 18));

    // Palindrome check
    println!("Is 'racecar' palindrome? {}", is_palindrome("racecar"));
    println!("Is 'hello' palindrome? {}", is_palindrome("hello"));

    // Tower of Hanoi
    println!("\nTower of Hanoi (3 disks):");
    tower_of_hanoi(3, 'A', 'C', 'B');
}

fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn sum_to(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        n + sum_to(n - 1)
    }
}

fn power(base: u64, exp: u64) -> u64 {
    if exp == 0 {
        1
    } else {
        base * power(base, exp - 1)
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() <= 1 {
        return true;
    }
    if bytes[0] != bytes[bytes.len() - 1] {
        return false;
    }
    is_palindrome(&s[1..s.len() - 1])
}

fn tower_of_hanoi(n: u32, from: char, to: char, aux: char) {
    if n == 1 {
        println!("  Move disk 1 from {} to {}", from, to);
    } else {
        tower_of_hanoi(n - 1, from, aux, to);
        println!("  Move disk {} from {} to {}", n, from, to);
        tower_of_hanoi(n - 1, aux, to, from);
    }
}
