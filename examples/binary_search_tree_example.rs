// Binary Search Tree (BST)
// Each node has at most two children: left (smaller) and right (larger).
// Inorder traversal yields sorted order. O(log n) average for search/insert/delete.

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct BstNode<T> {
    data: T,
    left: Option<Box<BstNode<T>>>,
    right: Option<Box<BstNode<T>>>,
}

#[derive(Debug)]
struct Bst<T> {
    root: Option<Box<BstNode<T>>>,
}

impl<T: Ord + Debug> Bst<T> {
    fn new() -> Self {
        Bst { root: None }
    }

    fn insert(&mut self, data: T) {
        let mut current = &mut self.root;
        while let Some(node) = current {
            match data.cmp(&node.data) {
                Ordering::Less => current = &mut node.left,
                Ordering::Greater => current = &mut node.right,
                Ordering::Equal => return, // no duplicates
            }
        }
        *current = Some(Box::new(BstNode {
            data,
            left: None,
            right: None,
        }));
    }

    fn search(&self, data: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            match data.cmp(&node.data) {
                Ordering::Less => current = &node.left,
                Ordering::Greater => current = &node.right,
                Ordering::Equal => return true,
            }
        }
        false
    }

    fn inorder(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::inorder_recursive(&self.root, &mut result);
        result
    }

    fn inorder_recursive<'a>(node: &'a Option<Box<BstNode<T>>>, result: &mut Vec<&'a T>) {
        if let Some(n) = node {
            Self::inorder_recursive(&n.left, result);
            result.push(&n.data);
            Self::inorder_recursive(&n.right, result);
        }
    }

    fn min(&self) -> Option<&T> {
        let mut current = &self.root;
        while let Some(node) = current {
            if node.left.is_some() {
                current = &node.left;
            } else {
                return Some(&node.data);
            }
        }
        None
    }

    fn max(&self) -> Option<&T> {
        let mut current = &self.root;
        while let Some(node) = current {
            if node.right.is_some() {
                current = &node.right;
            } else {
                return Some(&node.data);
            }
        }
        None
    }

    fn height(&self) -> usize {
        Self::height_recursive(&self.root)
    }

    fn height_recursive(node: &Option<Box<BstNode<T>>>) -> usize {
        match node {
            None => 0,
            Some(n) => 1 + Self::height_recursive(&n.left).max(Self::height_recursive(&n.right)),
        }
    }
}

fn main() {
    let mut bst = Bst::new();
    let values = vec![50, 30, 70, 20, 40, 60, 80, 10, 25, 55];

    for v in values {
        bst.insert(v);
    }

    println!("Inorder (sorted): {:?}", bst.inorder());
    println!("Min: {:?}", bst.min());
    println!("Max: {:?}", bst.max());
    println!("Height: {}", bst.height());
    println!("Search 25: {}", bst.search(&25));
    println!("Search 99: {}", bst.search(&99));

    let empty: Bst<i32> = Bst::new();
    println!("Empty BST min: {:?}", empty.min());
}
