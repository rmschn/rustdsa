// Graphs
// Collection of vertices (nodes) and edges connecting them.
// Implemented as adjacency list using HashMap.
// Traversals: BFS (breadth-first), DFS (depth-first).

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Graph {
    adjacency_list: HashMap<i32, Vec<i32>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: i32, v: i32) {
        self.adjacency_list.entry(u).or_default().push(v);
        self.adjacency_list.entry(v).or_default().push(u);
    }

    fn bfs(&self, start: i32) -> Vec<i32> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            result.push(node);
            if let Some(neighbors) = self.adjacency_list.get(&node) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        result
    }

    fn dfs(&self, start: i32) -> Vec<i32> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.dfs_recursive(start, &mut visited, &mut result);
        result
    }

    fn dfs_recursive(&self, node: i32, visited: &mut HashSet<i32>, result: &mut Vec<i32>) {
        if !visited.insert(node) {
            return;
        }
        result.push(node);
        if let Some(neighbors) = self.adjacency_list.get(&node) {
            for &neighbor in neighbors {
                self.dfs_recursive(neighbor, visited, result);
            }
        }
    }

    fn has_path(&self, start: i32, end: i32) -> bool {
        let visited = self.bfs(start);
        visited.contains(&end)
    }

    fn print(&self) {
        for (node, neighbors) in &self.adjacency_list {
            println!("  {} -> {:?}", node, neighbors);
        }
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(2, 5);
    graph.add_edge(3, 6);
    graph.add_edge(3, 7);

    println!("Graph adjacency list:");
    graph.print();

    println!("BFS from 1: {:?}", graph.bfs(1));
    println!("DFS from 1: {:?}", graph.dfs(1));
    println!("Path 1->7? {}", graph.has_path(1, 7));
    println!("Path 1->9? {}", graph.has_path(1, 9));
}
