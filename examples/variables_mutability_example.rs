// Variables and Mutability
// Variables in Rust are immutable by default. Use `mut` to make them mutable.
// Constants are always immutable and evaluated at compile time.
// Shadowing allows redeclaring a variable with the same name, potentially changing type.

fn main() {
    // Immutable variable (default)
    let x = 5;
    println!("Immutable x = {}", x);

    // Mutable variable
    let mut y = 10;
    println!("Mutable y before: {}", y);
    y += 5;
    println!("Mutable y after:  {}", y);

    // Constant
    const MAX_SPEED: u32 = 120;
    println!("Constant MAX_SPEED: {}", MAX_SPEED);

    // Shadowing - redeclare with same name
    let z = 42;
    let z = z + 8; // shadows previous z
    println!("Shadowed z: {}", z);

    // Shadowing with type change
    let label = "hello";
    let label = label.len(); // changes from &str to usize
    println!("Shadowed label as length: {}", label);
}
