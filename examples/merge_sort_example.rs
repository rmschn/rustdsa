// Merge Sort
// Divide-and-conquer: split array in half, recursively sort, then merge.
// Time: O(n log n) all cases (best/worst/avg).
// Space: O(n) auxiliary. Stable sort.

fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    let mid = n / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    merge(arr, mid);
}

fn merge<T: Ord + Clone>(arr: &mut [T], mid: usize) {
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut nums = vec![38, 27, 43, 3, 9, 82, 10];
    println!("Before: {:?}", nums);
    merge_sort(&mut nums);
    println!("After:  {:?}", nums);

    let mut desc = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    merge_sort(&mut desc);
    println!("Desc sorted: {:?}", desc);

    // Large-ish test
    let mut large: Vec<i32> = (0..20).rev().collect();
    merge_sort(&mut large);
    println!("Large rev sorted: {:?}", large);
}
