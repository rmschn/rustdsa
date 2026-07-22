// Dynamic Programming
// Solves problems by breaking them into overlapping subproblems.
// Two approaches: top-down (memoization) and bottom-up (tabulation).
// Key: optimal substructure + overlapping subproblems.

use std::collections::HashMap;

// Fibonacci: memoization (top-down)
fn fib_memo(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&val) = memo.get(&n) {
        return val;
    }
    let result = match n {
        0 => 0,
        1 => 1,
        _ => fib_memo(n - 1, memo) + fib_memo(n - 2, memo),
    };
    memo.insert(n, result);
    result
}

// Fibonacci: tabulation (bottom-up)
fn fib_tab(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut dp = vec![0u64; (n + 1) as usize];
    dp[1] = 1;
    for i in 2..=n as usize {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n as usize]
}

// 0/1 Knapsack: bottom-up
fn knapsack(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let n = weights.len();
    let mut dp = vec![vec![0; capacity + 1]; n + 1];

    for i in 1..=n {
        for w in 0..=capacity {
            if weights[i - 1] <= w {
                dp[i][w] = dp[i - 1][w].max(dp[i - 1][w - weights[i - 1]] + values[i - 1]);
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }
    dp[n][capacity]
}

// Longest Common Subsequence (LCS)
fn lcs(a: &str, b: &str) -> String {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let m = a_bytes.len();
    let n = b_bytes.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a_bytes[i - 1] == b_bytes[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // Reconstruct LCS string
    let mut result = Vec::new();
    let (mut i, mut j) = (m, n);
    while i > 0 && j > 0 {
        if a_bytes[i - 1] == b_bytes[j - 1] {
            result.push(a_bytes[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}

// Coin change (minimum coins)
fn min_coins(coins: &[usize], amount: usize) -> Option<usize> {
    let mut dp = vec![usize::MAX; amount + 1];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in coins {
            if coin <= i && dp[i - coin] != usize::MAX {
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
    }

    if dp[amount] == usize::MAX {
        None
    } else {
        Some(dp[amount])
    }
}

fn main() {
    // Fibonacci
    println!("Fibonacci(50) memo: {}", fib_memo(50, &mut HashMap::new()));
    println!("Fibonacci(50) tab:  {}", fib_tab(50));

    // 0/1 Knapsack
    let weights = vec![2, 3, 4, 5];
    let values = vec![3, 4, 5, 6];
    println!("Knapsack capacity 5: {}", knapsack(&weights, &values, 5));
    println!("Knapsack capacity 10: {}", knapsack(&weights, &values, 10));

    // LCS
    println!("LCS of 'ABCBDAB' and 'BDCAB': '{}'", lcs("ABCBDAB", "BDCAB"));

    // Min coins
    println!("Min coins for 11 with [1,2,5]: {:?}", min_coins(&[1, 2, 5], 11));
    println!("Min coins for 3 with [2]: {:?}", min_coins(&[2], 3));
}
