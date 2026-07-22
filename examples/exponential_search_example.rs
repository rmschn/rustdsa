// Exponential Search
// Finds range where target may exist by exponential jumps, then binary search on that range.
// Time: O(log n) worst/avg. Particularly useful for unbounded/infinite lists.
// Space: O(1) iterative.

fn exponential_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let n = arr.len();
    if n == 0 {
        return None;
    }

    if arr[0] == *target {
        return Some(0);
    }

    // Find range [2^(i-1), 2^i) where target may exist
    let mut i = 1;
    while i < n && arr[i] <= *target {
        i *= 2;
    }

    // Binary search in range [i/2, min(i, n))
    let low = i / 2;
    let high = std::cmp::min(i, n);
    binary_search_in_range(arr, target, low, high)
}

fn binary_search_in_range<T: Ord>(
    arr: &[T],
    target: &T,
    mut low: usize,
    mut high: usize,
) -> Option<usize> {
    while low < high {
        let mid = low + (high - low) / 2;
        match target.cmp(&arr[mid]) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => high = mid,
            std::cmp::Ordering::Greater => low = mid + 1,
        }
    }
    None
}

fn main() {
    let nums = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25];

    for &target in &[1, 15, 23, 30] {
        match exponential_search(&nums, &target) {
            Some(i) => println!("Found {} at index {}", target, i),
            None => println!("{} not found", target),
        }
    }

    // Small array
    let small = vec![5, 10, 15];
    println!("Small array - find 10: {:?}", exponential_search(&small, &10));
    println!("Small array - find 99: {:?}", exponential_search(&small, &99));
}
