// Linked List
// A sequential data structure where each element (node) points to the next.
// Singly linked list: each node has data and a pointer to the next node.
// Operations: insert (head/tail), delete, search. O(n) for access by index.

use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T: Debug> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }

    fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(new_node);
        self.len += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.data
        })
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn print(&self) {
        let mut current = &self.head;
        print!("List: ");
        while let Some(node) = current {
            print!("{:?} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    list.push_front(5);
    list.print();
    println!("Length: {}", list.len());

    let popped = list.pop_front();
    println!("Popped front: {:?}", popped);
    list.print();

    list.push_front(1);
    list.print();

    while !list.is_empty() {
        println!("Pop: {:?}", list.pop_front());
    }
}
