//Maximize the Number of Target Nodes After Connecting Trees I
use std::collections::{HashMap, VecDeque};

impl Solution {
    fn dfs(
        curr: i32,
        adj: &HashMap<i32, Vec<i32>>,
        d: i32,
        parent: i32,
    ) -> i32 {
        if d < 0 {
            return 0;
        }

        let mut count = 1;

        if let Some(neighbors) = adj.get(&curr) {
            for &neighbor in neighbors {
                if neighbor != parent {
                    count += Self::dfs(neighbor, adj, d - 1, curr);
                }
            }
        }

        count
    }

    fn find_count(edges: &[Vec<i32>], d: i32) -> Vec<i32> {
        let n = edges.len() as i32 + 1;
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();

        for edge in edges {
            let u = edge[0];
            let v = edge[1];
            adj.entry(u).or_default().push(v);
            adj.entry(v).or_default().push(u);
        }

        let mut res = vec![0; n as usize];

        for i in 0..n {
            res[i as usize] = Self::dfs(i, &adj, d, -1);
        }

        res
    }

    pub fn max_target_nodes(
        edges1: Vec<Vec<i32>>,
        edges2: Vec<Vec<i32>>,
        k: i32,
    ) -> Vec<i32> {
        let n = edges1.len() + 1;
        let res1 = Self::find_count(&edges1, k);
        let res2 = Self::find_count(&edges2, k - 1);

        let max_target_nodes_count = *res2.iter().max().unwrap_or(&0);

        res1
            .into_iter()
            .map(|val| val + max_target_nodes_count)
            .collect()
    }
}
