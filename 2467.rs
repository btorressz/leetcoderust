//2467. Most Profitable Path in a Tree

use std::collections::VecDeque;
use std::cmp::{max, min};

impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let n = edges.len() + 1;
        let mut graph = vec![vec![]; n];

        // Build the adjacency list
        for edge in &edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            graph[a].push(b);
            graph[b].push(a);
        }

        let mut ts = vec![n as i32; n];

        // DFS to find Bob's shortest path to node 0
        fn dfs1(node: usize, parent: i32, time: i32, graph: &Vec<Vec<usize>>, ts: &mut Vec<i32>) -> bool {
            if node == 0 {
                ts[node] = time;
                return true;
            }
            for &neighbor in &graph[node] {
                if neighbor as i32 != parent && dfs1(neighbor, node as i32, time + 1, graph, ts) {
                    ts[neighbor] = min(ts[neighbor], time + 1);
                    return true;
                }
            }
            false
        }

        dfs1(bob as usize, -1, 0, &graph, &mut ts);
        ts[bob as usize] = 0;

        let mut max_profit = i32::MIN;

        // DFS to calculate the max profit
        fn dfs2(node: usize, parent: i32, time: i32, value: i32, graph: &Vec<Vec<usize>>, ts: &Vec<i32>, amount: &Vec<i32>, max_profit: &mut i32) {
            let mut new_value = value;
            if time == ts[node] {
                new_value += amount[node] / 2;
            } else if time < ts[node] {
                new_value += amount[node];
            }

            let mut is_leaf = true;
            for &neighbor in &graph[node] {
                if neighbor as i32 != parent {
                    is_leaf = false;
                    dfs2(neighbor, node as i32, time + 1, new_value, graph, ts, amount, max_profit);
                }
            }

            if is_leaf {
                *max_profit = max(*max_profit, new_value);
            }
        }

        dfs2(0, -1, 0, 0, &graph, &ts, &amount, &mut max_profit);
        max_profit
    }
}
