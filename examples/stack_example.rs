// Stack
// LIFO (Last In, First Out) data structure.
// Core operations: push (add), pop (remove), peek (view top).
// Implemented here using Vec. O(1) for push/pop/peek.

#[derive(Debug)]
struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn len(&self) -> usize {
        self.elements.len()
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);
    println!("Stack: {:?}", stack.elements);
    println!("Peek: {:?}", stack.peek());
    println!("Pop: {:?}", stack.pop());
    println!("Pop: {:?}", stack.pop());
    println!("After pops: {:?}", stack.elements);

    stack.push(40);
    stack.push(50);
    println!("Len: {}", stack.len());

    // Use case: reversing
    let mut rev_stack = Stack::new();
    for i in 0..5 {
        rev_stack.push(i);
    }
    print!("Reversed: ");
    while let Some(val) = rev_stack.pop() {
        print!("{} ", val);
    }
    println!();

    // Use case: bracket matching
    println!("Brackets balanced: {}", is_balanced("({[]})"));
    println!("Brackets balanced: {}", is_balanced("({[})"));
}

fn is_balanced(s: &str) -> bool {
    let mut stack = Stack::new();
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
