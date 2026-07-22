// Quick Sort
// Divide-and-conquer: pick a pivot, partition around it, recursively sort partitions.
// Time: O(n log n) avg, O(n^2) worst (rare with good pivot selection).
// Space: O(log n) for recursion stack. In-place. Unstable.

fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_idx = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_idx);
    quick_sort(left);
    quick_sort(&mut right[1..]); // skip pivot
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let n = arr.len();
    // Use last element as pivot
    let pivot_idx = n - 1;
    let mut i = 0;

    for j in 0..pivot_idx {
        if arr[j] <= arr[pivot_idx] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_idx);
    i
}

// Median-of-three pivot selection for better performance
fn quick_sort_improved<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_idx = partition_improved(arr);
    let (left, right) = arr.split_at_mut(pivot_idx);
    quick_sort_improved(left);
    quick_sort_improved(&mut right[1..]);
}

fn partition_improved<T: Ord>(arr: &mut [T]) -> usize {
    let n = arr.len();
    let mid = n / 2;
    // Median of first, middle, last
    if arr[0] > arr[mid] {
        arr.swap(0, mid);
    }
    if arr[0] > arr[n - 1] {
        arr.swap(0, n - 1);
    }
    if arr[mid] > arr[n - 1] {
        arr.swap(mid, n - 1);
    }
    arr.swap(mid, n - 1);

    let pivot_idx = n - 1;
    let mut i = 0;
    for j in 0..pivot_idx {
        if arr[j] <= arr[pivot_idx] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_idx);
    i
}

fn main() {
    let mut nums = vec![10, 7, 8, 9, 1, 5];
    println!("Before: {:?}", nums);
    quick_sort(&mut nums);
    println!("After:  {:?}", nums);

    let mut nums2 = vec![3, 7, 8, 5, 2, 1, 9, 5, 4];
    quick_sort_improved(&mut nums2);
    println!("Improved: {:?}", nums2);

    // Already sorted
    let mut sorted = vec![1, 2, 3, 4, 5, 6, 7];
    quick_sort_improved(&mut sorted);
    println!("Sorted: {:?}", sorted);
}
