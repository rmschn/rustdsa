# RUSTDSA - Data Structures & Algorithms in Rust

A comprehensive collection of data structures and algorithms implemented in Rust, designed for learning and reference.

## Project Structure

```
src/
  main.rs           -- Entry point listing all examples
examples/
  *_example.rs      -- Runnable examples (one per topic)
```

## Running Examples

Each example is an independent binary. Run any with:

```bash
cargo run --example <name>
```

For example:

```bash
cargo run --example arrays_example
cargo run --example binary_search_tree_example
cargo run --example merge_sort_example
```

## Topics Covered

### Core Rust Concepts
| Example | File |
|---|---|
| Variables and Mutability | `variables_mutability_example.rs` |
| Data Types and Type Inference | `data_types_type_inference_example.rs` |
| Ownership and Borrowing | `ownership_borrowing_example.rs` |
| Slices and References | `slices_references_example.rs` |
| Structs and Enums | `structs_enums_example.rs` |
| Pattern Matching | `pattern_matching_example.rs` |
| Functions and Modules | `functions_modules_example.rs` |
| Generics and Traits | `generics_traits_example.rs` |
| Error Handling | `error_handling_example.rs` |
| Iterators and Closures | `iterators_closures_example.rs` |

### Data Structures
| Data Structure | File | Description |
|---|---|---|
| Arrays | `arrays_example.rs` | Fixed-size, stack-allocated collection |
| Vectors | `vectors_example.rs` | Growable, heap-allocated array |
| Strings | `strings_example.rs` | UTF-8 encoded, owned & borrowed |
| Linked List | `linked_list_example.rs` | Singly linked list with Box nodes |
| Stack | `stack_example.rs` | LIFO with bracket matching |
| Queue | `queue_example.rs` | FIFO with binary number generation |
| Hash Map | `hash_map_example.rs` | Key-value store with word count |
| Sets | `sets_example.rs` | Unique elements with set operations |
| Trees (General) | `trees_example.rs` | Hierarchical tree with traversals |
| Binary Search Tree | `binary_search_tree_example.rs` | Ordered BST with inorder traversal |
| Heap | `heap_example.rs` | Priority queue (min/max) |
| Graphs | `graphs_example.rs` | Adjacency list with BFS/DFS |

### Algorithms
| Algorithm | File | Complexity |
|---|---|---|
| Recursion | `recursion_example.rs` | Factorial, Fibonacci, Towers of Hanoi |
| **Sorting** | | |
| Bubble Sort | `bubble_sort_example.rs` | O(n²) |
| Selection Sort | `selection_sort_example.rs` | O(n²) |
| Insertion Sort | `insertion_sort_example.rs` | O(n²) |
| Merge Sort | `merge_sort_example.rs` | O(n log n) |
| Quick Sort | `quick_sort_example.rs` | O(n log n) avg |
| Heap Sort | `heap_sort_example.rs` | O(n log n) |
| Counting Sort | `counting_sort_example.rs` | O(n + k) |
| **Searching** | | |
| Linear Search | `linear_search_example.rs` | O(n) |
| Binary Search | `binary_search_example.rs` | O(log n) |
| Exponential Search | `exponential_search_example.rs` | O(log n) |
| Interpolation Search | `interpolation_search_example.rs` | O(log log n) avg |
| **Paradigms** | | |
| Dynamic Programming | `dynamic_programming_example.rs` | Fibonacci, Knapsack, LCS |
| Greedy Algorithms | `greedy_algorithm_example.rs` | Activity selection, Dijkstra |
| Backtracking | `backtracking_example.rs` | N-Queens, permutations |

### Low-Level
| Topic | File | Description |
|---|---|---|
| Bit Manipulation | `bit_manipulation_example.rs` | AND/OR/XOR, set/clear/toggle bits |
| Time Complexity | `time_complexity_example.rs` | O(1), O(log n), O(n), O(n log n), O(n²), O(2ⁿ) |
| Space Complexity | `space_complexity_example.rs` | O(1), O(n), O(n²) with examples |

## Requirements

- Rust (edition 2024)
- No external dependencies (uses only std library)

## DISCLAIMER

AI GENERATED CODE USING OPENCODE 

## License

MIT
