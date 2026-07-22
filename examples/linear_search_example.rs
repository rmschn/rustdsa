// Linear Search
// Sequentially checks each element until target found or list exhausted.
// Time: O(n) worst/avg, O(1) best (first element).
// Space: O(1). Works on unsorted data.

fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for (i, item) in arr.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

// Sentinel linear search (avoids bounds check on each iteration)
fn sentinel_linear_search<T: PartialEq>(arr: &mut [T], target: T) -> Option<usize>
where
    T: Clone,
{
    let n = arr.len();
    if n == 0 {
        return None;
    }
    let last = &arr[n - 1];
    if *last == target {
        return Some(n - 1);
    }

    // Place target at end as sentinel
    let tmp = arr[n - 1].clone();
    arr[n - 1] = target.clone();

    let mut i = 0;
    while arr[i] != target {
        i += 1;
    }

    // Restore last element
    arr[n - 1] = tmp;

    if i < n - 1 {
        Some(i)
    } else {
        None
    }
}

fn main() {
    let nums = vec![5, 3, 8, 1, 9, 2, 7];
    let target = 8;
    match linear_search(&nums, &target) {
        Some(i) => println!("Found {} at index {}", target, i),
        None => println!("{} not found", target),
    }

    let target2 = 10;
    match linear_search(&nums, &target2) {
        Some(i) => println!("Found {} at index {}", target2, i),
        None => println!("{} not found", target2),
    }

    // With strings
    let words = vec!["apple", "banana", "cherry", "date"];
    match linear_search(&words, &"cherry") {
        Some(i) => println!("Found 'cherry' at index {}", i),
        None => println!("Not found"),
    }

    // Sentinel search
    let mut data = vec![4, 2, 7, 1, 9];
    match sentinel_linear_search(&mut data, 7) {
        Some(i) => println!("Sentinel found 7 at index {}", i),
        None => println!("Sentinel: 7 not found"),
    }
}
