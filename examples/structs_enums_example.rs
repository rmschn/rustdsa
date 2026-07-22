// Structs and Enums
// Structs group related data. Three types: named-field, tuple struct, unit struct.
// Enums define a type with variants. Can hold data (like tagged unions).
// `Option` and `Result` are built-in enums.

fn main() {
    // Named-field struct
    struct User {
        username: String,
        email: String,
        active: bool,
    }
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };
    println!("User: {} ({}) active: {}", user1.username, user1.email, user1.active);

    // Tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);

    // Unit struct
    struct AlwaysEqual;
    let _unit = AlwaysEqual;

    // Enum (defined outside main for function use)
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Print via helper
    print_ip(&home);
    print_ip(&loopback);

    // Option enum (built-in)
    let some_num = Some(5);
    let absent: Option<i32> = None;
    println!("Option some: {:?}, none: {:?}", some_num, absent);

    // Struct with method
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
    }
    let rect = Rectangle::new(30, 50);
    println!("Rectangle area: {}", rect.area());
}

fn print_ip(ip: &IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
