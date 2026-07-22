// Greedy Algorithms
// Makes locally optimal choice at each step hoping for global optimum.
// Works when the problem has optimal substructure and greedy choice property.
// Examples: coin change (canonical systems), activity selection, Huffman coding.

// Activity Selection: select max non-overlapping activities
fn activity_selection(starts: &[usize], finishes: &[usize]) -> Vec<usize> {
    let n = starts.len();
    let mut activities: Vec<(usize, usize, usize)> =
        (0..n).map(|i| (finishes[i], starts[i], i)).collect();
    activities.sort(); // sort by finish time

    let mut selected = Vec::new();
    let mut last_finish = 0;

    for &(finish, start, idx) in &activities {
        if start >= last_finish {
            selected.push(idx);
            last_finish = finish;
        }
    }
    selected
}

// Coin change: greedy for canonical systems (e.g., USD)
fn coin_change_greedy(amount: u32, coins: &[u32]) -> Option<Vec<u32>> {
    let mut coins_sorted = coins.to_vec();
    coins_sorted.sort_by(|a, b| b.cmp(a)); // descending

    let mut remaining = amount;
    let mut result = Vec::new();

    for &coin in &coins_sorted {
        while remaining >= coin {
            remaining -= coin;
            result.push(coin);
        }
    }

    if remaining == 0 {
        Some(result)
    } else {
        None
    }
}

// Fractional Knapsack: can take fractions of items
fn fractional_knapsack(weights: &[f64], values: &[f64], capacity: f64) -> f64 {
    let n = weights.len();
    let mut items: Vec<(f64, f64, f64)> = (0..n)
        .map(|i| (values[i] / weights[i], weights[i], values[i]))
        .collect();
    items.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let mut total_value = 0.0;
    let mut remaining = capacity;

    for &(ratio, weight, value) in &items {
        if remaining <= 0.0 {
            break;
        }
        if weight <= remaining {
            total_value += value;
            remaining -= weight;
        } else {
            total_value += ratio * remaining;
            remaining = 0.0;
        }
    }
    total_value
}

// Dijkstra's (simplified) - shortest path
// Implemented using straightforward approach (no priority queue for simplicity)
fn dijkstra_shortest_paths(
    graph: &[Vec<(usize, u32)>],
    start: usize,
) -> Vec<u32> {
    let n = graph.len();
    let mut dist = vec![u32::MAX; n];
    let mut visited = vec![false; n];
    dist[start] = 0;

    for _ in 0..n {
        // Find unvisited vertex with minimum distance
        let mut u = None;
        let mut min_dist = u32::MAX;
        for v in 0..n {
            if !visited[v] && dist[v] < min_dist {
                min_dist = dist[v];
                u = Some(v);
            }
        }

        let u = match u {
            Some(v) => v,
            None => break,
        };
        visited[u] = true;

        for &(v, w) in &graph[u] {
            if !visited[v] && dist[u] != u32::MAX && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }
    dist
}

fn main() {
    // Activity selection
    let starts = vec![1, 3, 0, 5, 8, 5];
    let finishes = vec![2, 4, 6, 7, 9, 9];
    println!("Selected activities: {:?}", activity_selection(&starts, &finishes));

    // Coin change
    let coins = vec![25, 10, 5, 1];
    println!("Coins for 67¢: {:?}", coin_change_greedy(67, &coins));

    // Fractional knapsack
    let weights = vec![10.0, 20.0, 30.0];
    let values = vec![60.0, 100.0, 120.0];
    println!("Fractional knapsack (cap=50): {:.2}", fractional_knapsack(&weights, &values, 50.0));

    // Dijkstra's
    let graph = vec![
        vec![(1, 4), (2, 1)],
        vec![(3, 1)],
        vec![(1, 2), (3, 5)],
        vec![],
    ];
    println!("Dijkstra from 0: {:?}", dijkstra_shortest_paths(&graph, 0));
}
