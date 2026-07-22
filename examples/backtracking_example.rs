// Backtracking
// Systematic trial-and-error: build candidates incrementally, abandon (backtrack) when invalid.
// Common for constraint satisfaction: N-Queens, Sudoku, permutations.

// N-Queens: place N queens on NxN board so none attack each other
fn solve_nqueens(n: usize) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut board = vec![vec!['.'; n]; n];
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n - 1]; // row + col
    let mut diag2 = vec![false; 2 * n - 1]; // row + n - col - 1

    backtrack_nqueens(
        0, n, &mut board, &mut cols, &mut diag1, &mut diag2, &mut result,
    );
    result
}

fn backtrack_nqueens(
    row: usize,
    n: usize,
    board: &mut Vec<Vec<char>>,
    cols: &mut Vec<bool>,
    diag1: &mut Vec<bool>,
    diag2: &mut Vec<bool>,
    result: &mut Vec<Vec<String>>,
) {
    if row == n {
        result.push(board.iter().map(|r| r.iter().collect()).collect());
        return;
    }

    for col in 0..n {
        let d1 = row + col;
        let d2 = row + n - col - 1;
        if cols[col] || diag1[d1] || diag2[d2] {
            continue;
        }

        board[row][col] = 'Q';
        cols[col] = true;
        diag1[d1] = true;
        diag2[d2] = true;

        backtrack_nqueens(row + 1, n, board, cols, diag1, diag2, result);

        board[row][col] = '.';
        cols[col] = false;
        diag1[d1] = false;
        diag2[d2] = false;
    }
}

// Generate all permutations of a vector
fn permute<T: Clone>(arr: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut current = arr.to_vec();
    backtrack_permute(0, &mut current, &mut result);
    result
}

fn backtrack_permute<T: Clone>(start: usize, arr: &mut Vec<T>, result: &mut Vec<Vec<T>>) {
    if start == arr.len() {
        result.push(arr.clone());
        return;
    }
    for i in start..arr.len() {
        arr.swap(start, i);
        backtrack_permute(start + 1, arr, result);
        arr.swap(start, i);
    }
}

// Subset sum: find subsets that sum to target
fn subset_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    backtrack_subset_sum(nums, target, 0, &mut current, 0, &mut result);
    result
}

fn backtrack_subset_sum(
    nums: &[i32],
    target: i32,
    start: usize,
    current: &mut Vec<i32>,
    sum: i32,
    result: &mut Vec<Vec<i32>>,
) {
    if sum == target {
        result.push(current.clone());
        return;
    }
    for i in start..nums.len() {
        if sum + nums[i] <= target {
            current.push(nums[i]);
            backtrack_subset_sum(nums, target, i + 1, current, sum + nums[i], result);
            current.pop();
        }
    }
}

fn main() {
    // N-Queens
    let solutions = solve_nqueens(4);
    println!("4-Queens solutions ({}):", solutions.len());
    for (i, sol) in solutions.iter().enumerate() {
        println!("Solution {}:", i + 1);
        for row in sol {
            println!("  {}", row);
        }
    }

    // Permutations
    let perms = permute(&[1, 2, 3]);
    println!("\nPermutations of [1,2,3]: {:?}", perms);

    // Subset sum
    let subsets = subset_sum(&[3, 1, 2, 4, 5], 7);
    println!("Subsets summing to 7: {:?}", subsets);
}
