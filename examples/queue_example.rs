// Queue
// FIFO (First In, First Out) data structure.
// Core operations: enqueue (add to back), dequeue (remove from front), peek.
// Implemented with VecDeque for O(1) amortized operations.

use std::collections::VecDeque;

#[derive(Debug)]
struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            elements: VecDeque::new(),
        }
    }

    fn enqueue(&mut self, item: T) {
        self.elements.push_back(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.front()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn len(&self) -> usize {
        self.elements.len()
    }
}

fn main() {
    let mut queue: Queue<i32> = Queue::new();

    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);
    println!("Queue: {:?}", queue.elements);
    println!("Peek: {:?}", queue.peek());
    println!("Dequeue: {:?}", queue.dequeue());
    println!("Dequeue: {:?}", queue.dequeue());
    println!("After dequeues: {:?}", queue.elements);

    queue.enqueue(40);
    queue.enqueue(50);
    println!("Len: {}", queue.len());

    // Use case: processing tasks in order
    let mut tasks = Queue::new();
    for i in 1..=5 {
        tasks.enqueue(format!("Task {}", i));
    }
    println!("Processing tasks:");
    while let Some(task) = tasks.dequeue() {
        println!("  Processing {}", task);
    }

    // Use case: binary numbers generation
    println!("Binary numbers 1..10:");
    generate_binary(10);
}

fn generate_binary(n: u32) {
    let mut q = Queue::new();
    q.enqueue("1".to_string());
    for _ in 0..n {
        if let Some(current) = q.dequeue() {
            print!("{} ", current);
            q.enqueue(format!("{}0", current));
            q.enqueue(format!("{}1", current));
        }
    }
    println!();
}
