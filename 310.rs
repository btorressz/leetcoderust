//310 Minimum Height Trees
use std::collections::VecDeque;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0]; // single node case
        }
        
        let mut adj = vec![vec![]; n as usize]; // adjacency list
        for edge in edges {
            adj[edge[0] as usize].push(edge[1] as usize);
            adj[edge[1] as usize].push(edge[0] as usize);
        }
        
        let mut leaves = VecDeque::new(); // queue for leaves
        let mut degree = vec![0; n as usize];
        
        for i in 0..n as usize {
            degree[i] = adj[i].len();
            if degree[i] == 1 {
                leaves.push_back(i); // add initial leaves
            }
        }
        
        let mut remaining_nodes = n as usize;
        
        while remaining_nodes > 2 {
            remaining_nodes -= leaves.len(); // remove current leaves
            
            for _ in 0..leaves.len() {
                let leaf = leaves.pop_front().unwrap();
                for &neighbor in &adj[leaf] {
                    degree[neighbor] -= 1;
                    if degree[neighbor] == 1 {
                        leaves.push_back(neighbor); // add new leaves
                    }
                }
            }
        }
        
        leaves.into_iter().map(|x| x as i32).collect() // remaining nodes
    }
}
