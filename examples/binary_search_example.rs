// Binary Search
// Efficient search on sorted array. Repeatedly divides search interval in half.
// Time: O(log n) worst/avg, O(1) best.
// Space: O(1) iterative, O(log n) recursive (stack).

fn binary_search_recursive<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let mid = arr.len() / 2;
    match target.cmp(&arr[mid]) {
        std::cmp::Ordering::Equal => Some(mid),
        std::cmp::Ordering::Less => binary_search_recursive(&arr[..mid], target),
        std::cmp::Ordering::Greater => {
            binary_search_recursive(&arr[mid + 1..], target)
                .map(|i| i + mid + 1)
        }
    }
}

fn binary_search_iterative<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

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

// Lower bound: first index where value >= target
fn lower_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] < *target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

fn main() {
    let nums = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];

    for &target in &[7, 19, 1, 20] {
        match binary_search_recursive(&nums, &target) {
            Some(i) => println!("Recursive: found {} at {}", target, i),
            None => println!("Recursive: {} not found", target),
        }
        match binary_search_iterative(&nums, &target) {
            Some(i) => println!("Iterative: found {} at {}", target, i),
            None => println!("Iterative: {} not found", target),
        }
    }

    // Lower bound
    let sorted = vec![1, 2, 2, 2, 3, 4, 5];
    println!("Lower bound of 2: {}", lower_bound(&sorted, &2));
    println!("Lower bound of 6: {}", lower_bound(&sorted, &6)); // len (not found sentinel)
}
