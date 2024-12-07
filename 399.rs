//399. Evaluate Division
// I used BFS to traverse a weighted graph representing division equations 
// and calculate the result of division queries based on the graph's paths.

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        
        let n = equations.len();
        let m = queries.len();

        // Build the graph with both directions: A/B = 3 means B/A = 1/3
        for i in 0..n {
            let a = &equations[i][0];
            let b = &equations[i][1];
            let value = values[i];
            graph.entry(a.clone()).or_insert_with(Vec::new).push((b.clone(), value));
            graph.entry(b.clone()).or_insert_with(Vec::new).push((a.clone(), 1.0 / value));
        }

        let mut result = vec![-1.0; m]; // Initialize the result array with -1.0

        // Process each query
        for (i, query) in queries.iter().enumerate() {
            let start = &query[0];
            let dest = &query[1];

            // Skip if either start or destination doesn't exist in the graph
            if !graph.contains_key(start) || !graph.contains_key(dest) {
                continue;
            }

            let mut seen = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((start.clone(), 1.0)); // Start BFS from the start node

            // Perform BFS to find the path from start to destination
            while let Some((node, prev_division)) = queue.pop_front() {
                if node == *dest {
                    result[i] = prev_division;
                    break; // Found the destination, no need to search further
                }

                // Explore neighbors of the current node
                if !seen.contains(&node) {
                    seen.insert(node.clone());
                    if let Some(neighbors) = graph.get(&node) {
                        for (neighbor, value) in neighbors {
                            if !seen.contains(neighbor) {
                                queue.push_back((neighbor.clone(), prev_division * value));
                            }
                        }
                    }
                }
            }
        }

        result
    }
}
