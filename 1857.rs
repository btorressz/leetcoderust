//1857. Largest Color Value in a Directed Graph

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        for edge in edges {
            graph.entry(edge[0]).or_default().push(edge[1]);
        }

        let mut visited = vec![0; n];

        // Cycle detection using DFS
        fn has_cycle(
            node: i32,
            graph: &HashMap<i32, Vec<i32>>,
            visited: &mut Vec<i32>,
        ) -> bool {
            if visited[node as usize] == 1 {
                return true;
            }
            if visited[node as usize] == 2 {
                return false;
            }
            visited[node as usize] = 1;
            for &next in graph.get(&node).unwrap_or(&vec![]) {
                if has_cycle(next, graph, visited) {
                    return true;
                }
            }
            visited[node as usize] = 2;
            false
        }

        for i in 0..n {
            if has_cycle(i as i32, &graph, &mut visited) {
                return -1;
            }
        }

        // Memoized DFS to track max color counts
        let mut memo: HashMap<i32, Vec<i32>> = HashMap::new();
        let color_bytes = colors.as_bytes();

        fn dfs(
            node: i32,
            graph: &HashMap<i32, Vec<i32>>,
            color_bytes: &[u8],
            memo: &mut HashMap<i32, Vec<i32>>,
        ) -> Vec<i32> {
            if let Some(cached) = memo.get(&node) {
                return cached.clone();
            }

            let mut count = vec![0; 26];
            for &next in graph.get(&node).unwrap_or(&vec![]) {
                let next_count = dfs(next, graph, color_bytes, memo);
                for i in 0..26 {
                    count[i] = count[i].max(next_count[i]);
                }
            }

            let c = (color_bytes[node as usize] - b'a') as usize;
            count[c] += 1;
            memo.insert(node, count.clone());
            count
        }

        let mut res = 0;
        for i in 0..n {
            let counts = dfs(i as i32, &graph, &color_bytes, &mut memo);
            res = res.max(*counts.iter().max().unwrap());
        }

        res
    }
}
