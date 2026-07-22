// Functions and Modules
// Functions use `fn` keyword. Return type after `->`.
// Modules organize code. `mod` declares, `pub` makes visible.
// `use` brings items into scope.

mod math {
    // Public function - accessible from outside
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // Private function - only within this module
    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }

    // Public wrapper calling private
    pub fn subtract(a: i32, b: i32) -> i32 {
        sub(a, b)
    }

    // Nested module
    pub mod advanced {
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }

        pub fn factorial(n: u64) -> u64 {
            match n {
                0 | 1 => 1,
                _ => n * factorial(n - 1),
            }
        }
    }
}

// Function with expressions vs statements
fn is_even(n: i32) -> bool {
    n % 2 == 0 // no semicolon = return expression
}

// Function diverge type (never returns)
fn diverging() -> ! {
    panic!("This function never returns");
}

fn main() {
    // Using the modules
    println!("Add: {}", math::add(5, 3));
    println!("Subtract: {}", math::subtract(10, 4));
    println!("Multiply: {}", math::advanced::multiply(6, 7));
    println!("Factorial(5): {}", math::advanced::factorial(5));

    // Bring into scope with use
    use math::advanced::multiply as mul;
    println!("With use + alias: {}", mul(3, 4));

    // Expression-based function
    println!("Is 4 even? {}", is_even(4));
    println!("Is 5 even? {}", is_even(5));

    // Function pointers
    let fn_ptr: fn(i32, i32) -> i32 = math::add;
    println!("Function pointer: {}", fn_ptr(100, 200));
}
