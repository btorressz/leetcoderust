//2097. Valid Arrangement of Pairs

use std::collections::HashMap;

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        let mut count: HashMap<i32, i32> = HashMap::new();

        // Build the graph and count degrees
        for pair in pairs.iter() {
            let start = pair[0];
            let end = pair[1];
            
            graph.entry(start).or_insert(Vec::new()).push(pair.clone());
            *count.entry(start).or_insert(0) += 1;
            *count.entry(end).or_insert(0) -= 1;
        }

        // Find the starting node
        let mut start = pairs[0][0];
        for (&key, &value) in count.iter() {
            if value == 1 {
                start = key;
                break;
            }
        }

        let mut result = Vec::new();

        // DFS to build the result
        fn dfs(node: i32, graph: &mut HashMap<i32, Vec<Vec<i32>>>, result: &mut Vec<Vec<i32>>) {
            while let Some(pair) = graph.get_mut(&node).and_then(|edges| edges.pop()) {
                dfs(pair[1], graph, result);
                result.push(pair);
            }
        }

        dfs(start, &mut graph, &mut result);

        // Reverse to get the correct order
        result.reverse();
        result
    }
}
