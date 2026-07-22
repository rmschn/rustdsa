// Insertion Sort
// Builds sorted array one element at a time by inserting each new element into its correct position.
// Time: O(n^2) worst/avg, O(n) best (already sorted).
// Space: O(1) in-place. Good for small or nearly-sorted datasets.

fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn insertion_sort_shift<T: Ord + Clone>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i].clone();
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        arr[j] = key;
    }
}

fn main() {
    let mut nums = vec![12, 11, 13, 5, 6];
    println!("Before: {:?}", nums);
    insertion_sort(&mut nums);
    println!("After:  {:?}", nums);

    // Nearly sorted - insertion sort excels here
    let mut nearly = vec![1, 2, 4, 3, 5, 6, 8, 7];
    insertion_sort_shift(&mut nearly);
    println!("Nearly sorted: {:?}", nearly);

    // Strings
    let mut strs = vec!["dog", "cat", "bird", "fish"];
    insertion_sort(&mut strs);
    println!("Sorted strs: {:?}", strs);
}
