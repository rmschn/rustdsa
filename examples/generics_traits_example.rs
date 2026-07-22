// Generics and Traits
// Generics allow code to work with multiple types. <T> syntax.
// Traits define shared behavior. Similar to interfaces in other languages.
// Trait bounds constrain generic types.

use std::fmt::Display;

// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct
struct Pair<T> {
    x: T,
    y: T,
}

// Generic impl with trait bound
impl<T: Display> Pair<T> {
    fn display(&self) {
        println!("Pair({}, {})", self.x, self.y);
    }
}

// Define a trait
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn author(&self) -> String {
        String::from("Unknown")
    }
}

// Implement trait on a type
struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}

struct Tweet {
    username: String,
    text: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.text)
    }

    fn author(&self) -> String {
        self.username.clone()
    }
}

// Trait bound on function
fn notify(item: &impl Summary) {
    println!("Breaking: {}", item.summarize());
}

// Trait bound with where clause
fn print_author<T>(item: &T)
where
    T: Summary,
{
    println!("Author: {}", item.author());
}

fn main() {
    // Generic function
    let nums = vec![34, 50, 25, 100, 65];
    println!("Largest num: {}", largest(&nums));

    let words = vec!["apple", "banana", "cherry"];
    println!("Largest word: {}", largest(&words));

    // Generic struct
    let p = Pair { x: 10, y: 20 };
    p.display();

    // Traits
    let article = Article {
        title: String::from("Rust Programming"),
        content: String::from("..."),
    };
    let tweet = Tweet {
        username: String::from("rustacean"),
        text: String::from("Loving Rust!"),
    };

    notify(&article);
    notify(&tweet);
    print_author(&tweet);
}
