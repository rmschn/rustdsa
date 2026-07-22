// Bubble Sort
// Repeatedly steps through the list, compares adjacent elements, and swaps them if out of order.
// Time: O(n^2) worst/avg, O(n) best (already sorted, with optimization).
// Space: O(1) in-place.

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut nums = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Before: {:?}", nums);
    bubble_sort(&mut nums);
    println!("After:  {:?}", nums);

    let mut words = vec!["banana", "apple", "cherry", "date"];
    bubble_sort(&mut words);
    println!("Sorted words: {:?}", words);

    // Analyze performance
    let mut nearly_sorted = vec![1, 2, 3, 4, 6, 5, 7];
    bubble_sort(&mut nearly_sorted);
    println!("Nearly sorted: {:?}", nearly_sorted);
}
