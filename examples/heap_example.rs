// Heap (Priority Queue)
// A complete binary tree where each parent is >= (max-heap) or <= (min-heap) its children.
// BinaryHeap in Rust is a max-heap by default. O(log n) push/pop, O(1) peek.

use std::collections::BinaryHeap;

fn main() {
    // Max-heap (default)
    let mut max_heap = BinaryHeap::new();
    max_heap.push(10);
    max_heap.push(5);
    max_heap.push(20);
    max_heap.push(15);
    println!("Max-heap: {:?}", max_heap);
    println!("Peek (max): {:?}", max_heap.peek());

    // Pop returns largest
    println!("Pop: {:?}", max_heap.pop());
    println!("Pop: {:?}", max_heap.pop());
    println!("After pops: {:?}", max_heap);

    // Min-heap using std::cmp::Reverse
    use std::cmp::Reverse;
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(10));
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(20));
    println!("Min-heap pop: {:?}", min_heap.pop().map(|r| r.0));
    println!("Min-heap pop: {:?}", min_heap.pop().map(|r| r.0));

    // Heap from vec
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let heap: BinaryHeap<i32> = nums.into_iter().collect();
    println!("Heap from vec: {:?}", heap);

    // Sorting with heap (heapsort)
    let mut h = BinaryHeap::from(vec![5, 3, 8, 1, 2, 7]);
    let mut sorted = Vec::new();
    while let Some(v) = h.pop() {
        sorted.push(v);
    }
    println!("Heapsorted (descending): {:?}", sorted);

    // Use case: top K elements
    let data = vec![10, 30, 20, 5, 40, 35, 15];
    let k = 3;
    let mut top_k = BinaryHeap::new();
    for &x in &data {
        top_k.push(Reverse(x));
        if top_k.len() > k {
            top_k.pop();
        }
    }
    let top_k_values: Vec<i32> = top_k.into_iter().map(|r| r.0).collect();
    println!("Top {} smallest: {:?}", k, top_k_values);
}
