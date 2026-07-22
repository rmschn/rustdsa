// Interpolation Search
// Improved binary search for uniformly distributed sorted data.
// Probes position based on value, not midpoint.
// Time: O(log log n) avg, O(n) worst (non-uniform distribution).
// Space: O(1).

fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
    let n = arr.len();
    if n == 0 {
        return None;
    }

    let mut low: usize = 0;
    let mut high: usize = n - 1;

    while low <= high && target >= arr[low] && target <= arr[high] {
        if low == high {
            return if arr[low] == target { Some(low) } else { None };
        }

        // Probe position using interpolation formula
        let probe = low
            + (((target - arr[low]) as f64 / (arr[high] - arr[low]) as f64)
                * (high - low) as f64) as usize;

        if probe >= n {
            return None;
        }

        match target.cmp(&arr[probe]) {
            std::cmp::Ordering::Equal => return Some(probe),
            std::cmp::Ordering::Less => {
                if probe == 0 {
                    return None;
                }
                high = probe - 1;
            }
            std::cmp::Ordering::Greater => low = probe + 1,
        }
    }
    None
}

fn main() {
    // Uniformly distributed data - interpolation search excels
    let nums = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    for &target in &[10, 50, 100, 55] {
        match interpolation_search(&nums, target) {
            Some(i) => println!("Found {} at index {}", target, i),
            None => println!("{} not found", target),
        }
    }

    // Non-uniform (not ideal for interpolation search)
    let uneven = vec![1, 2, 3, 4, 5, 100, 200, 300, 400, 500];
    match interpolation_search(&uneven, 100) {
        Some(i) => println!("Found 100 at index {} in uneven array", i),
        None => println!("100 not found"),
    }

    // Single element
    let single = vec![42];
    println!("Single element: {:?}", interpolation_search(&single, 42));
    println!("Single element: {:?}", interpolation_search(&single, 41));
}
