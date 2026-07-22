// Heap Sort
// Builds a max-heap, then repeatedly extracts the max to produce sorted order.
// Time: O(n log n) all cases (best/worst/avg).
// Space: O(1) in-place. Unstable sort.

fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // Build max-heap
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // Extract elements from heap
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn main() {
    let mut nums = vec![12, 11, 13, 5, 6, 7];
    println!("Before: {:?}", nums);
    heap_sort(&mut nums);
    println!("After:  {:?}", nums);

    let mut desc = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    heap_sort(&mut desc);
    println!("Desc sorted: {:?}", desc);

    let mut rand = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    heap_sort(&mut rand);
    println!("Rand sorted: {:?}", rand);
}
