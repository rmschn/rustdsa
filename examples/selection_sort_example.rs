// Selection Sort
// Divides array into sorted and unsorted parts. Repeatedly selects the smallest from unsorted.
// Time: O(n^2) all cases (best/worst/avg).
// Space: O(1) in-place. Makes fewer swaps than bubble sort (O(n) swaps).

fn selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            arr.swap(i, min_idx);
        }
    }
}

fn main() {
    let mut nums = vec![29, 10, 14, 37, 13, 33, 20];
    println!("Before: {:?}", nums);
    selection_sort(&mut nums);
    println!("After:  {:?}", nums);

    let mut chars = vec!['z', 'a', 'm', 'b', 'y'];
    selection_sort(&mut chars);
    println!("Sorted chars: {:?}", chars);

    // Demo with many duplicates
    let mut dupes = vec![5, 3, 5, 1, 3, 5, 1];
    selection_sort(&mut dupes);
    println!("With duplicates: {:?}", dupes);
}
