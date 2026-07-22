// Bit Manipulation
// Operating directly on bits. Efficient for certain operations.
// Common ops: AND, OR, XOR, NOT, shifts, set/clear/toggle/test bits.

fn main() {
    let a: u8 = 0b1100; // 12
    let b: u8 = 0b1010; // 10

    // Basic operations
    println!("a = {:08b} ({})", a, a);
    println!("b = {:08b} ({})", b, b);
    println!("a & b  = {:08b} ({})", a & b, a & b);
    println!("a | b  = {:08b} ({})", a | b, a | b);
    println!("a ^ b  = {:08b} ({})", a ^ b, a ^ b);
    println!("!a     = {:08b} ({})", !a, !a as u8);
    println!("a << 2 = {:08b} ({})", a << 2, a << 2);
    println!("a >> 2 = {:08b} ({})", a >> 2, a >> 2);

    // Check if power of 2
    println!("\nPower of 2 checks:");
    println!("16 is power of 2: {}", is_power_of_two(16));
    println!("12 is power of 2: {}", is_power_of_two(12));

    // Count set bits
    println!("\nPopcount:");
    println!("Bits in 0b1101 (13): {}", count_bits(13));
    println!("Bits in 0b1111 (15): {}", count_bits(15));

    // Get/set/clear/toggle individual bits
    let mut x: u8 = 0b0000_1010;
    println!("\nx = {:08b}", x);
    println!("Bit 3: {}", get_bit(x, 3));
    println!("Bit 2: {}", get_bit(x, 2));

    x = set_bit(x, 0);
    println!("After set bit 0: {:08b}", x);

    x = clear_bit(x, 3);
    println!("After clear bit 3: {:08b}", x);

    x = toggle_bit(x, 1);
    println!("After toggle bit 1: {:08b}", x);

    // Find unique element (XOR trick)
    let arr = vec![4, 2, 3, 2, 4];
    println!("\nUnique in {:?}: {}", arr, find_unique(&arr));

    // Swap without temp
    let (x, y) = (5, 9);
    let (x, y) = swap_xor(x, y);
    println!("\nSwap XOR: 5,9 -> {},{}", x, y);

    // Isolate rightmost set bit
    let n: u8 = 0b0101_1000;
    let rightmost = n & (!n).wrapping_add(1);
    println!("\nRightmost set bit of {:08b}: {:08b}", n, rightmost);
}

fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

fn count_bits(mut n: u32) -> u32 {
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

fn get_bit(n: u8, bit: u8) -> bool {
    (n >> bit) & 1 == 1
}

fn set_bit(n: u8, bit: u8) -> u8 {
    n | (1 << bit)
}

fn clear_bit(n: u8, bit: u8) -> u8 {
    n & !(1 << bit)
}

fn toggle_bit(n: u8, bit: u8) -> u8 {
    n ^ (1 << bit)
}

fn find_unique(arr: &[i32]) -> i32 {
    arr.iter().fold(0, |acc, &x| acc ^ x)
}

fn swap_xor(a: i32, b: i32) -> (i32, i32) {
    // x = a ^ b, y = (a ^ b) ^ b = a, x = (a ^ b) ^ a = b
    let x = a ^ b;
    let y = x ^ b;
    let x = x ^ y;
    (x, y)
}
