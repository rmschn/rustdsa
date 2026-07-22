// Counting Sort
// Non-comparison integer sort. Counts occurrences, then builds sorted output.
// Time: O(n + k) where k is range of input. Space: O(k).
// Only works for non-negative integers with limited range.

fn counting_sort(arr: &mut [u32]) {
    if arr.is_empty() {
        return;
    }

    let max = *arr.iter().max().unwrap() as usize;
    let mut count = vec![0; max + 1];

    // Count occurrences
    for &val in arr.iter() {
        count[val as usize] += 1;
    }

    // Cumulative count (positions)
    for i in 1..count.len() {
        count[i] += count[i - 1];
    }

    // Build output (stable version)
    let mut output = vec![0; arr.len()];
    for &val in arr.iter().rev() {
        let pos = &mut count[val as usize];
        *pos -= 1;
        output[*pos] = val;
    }

    arr.copy_from_slice(&output);
}

// For negative numbers, shift range
fn counting_sort_signed(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    let range = (max - min + 1) as usize;

    let mut count = vec![0; range];

    for &val in arr.iter() {
        count[(val - min) as usize] += 1;
    }

    let mut idx = 0;
    for (i, &c) in count.iter().enumerate() {
        for _ in 0..c {
            arr[idx] = i as i32 + min;
            idx += 1;
        }
    }
}

fn main() {
    let mut nums = vec![4, 2, 2, 8, 3, 3, 1];
    println!("Before: {:?}", nums);
    counting_sort(&mut nums);
    println!("After:  {:?}", nums);

    // Larger range
    let mut nums2 = vec![9, 3, 7, 1, 5, 8, 2, 6, 4, 0];
    counting_sort(&mut nums2);
    println!("Range: {:?}", nums2);

    // Signed version
    let mut signed = vec![4, -2, 0, 3, -1, 2, -3, 1];
    println!("Signed before: {:?}", signed);
    counting_sort_signed(&mut signed);
    println!("Signed after:  {:?}", signed);
}
