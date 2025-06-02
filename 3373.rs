//3373. Maximize the Number of Target Nodes After Connecting Trees II
use std::collections::VecDeque;

impl Solution {
    fn bfs(start: usize, adj: &Vec<Vec<usize>>, mut included: Option<&mut Vec<bool>>) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((start, usize::MAX)); 
        let mut count = 0;
        let mut level = 0;

        while !queue.is_empty() {
            let size = queue.len();

            if level % 2 == 0 {
                count += size;
            }

            for _ in 0..size {
                let (curr, parent) = queue.pop_front().unwrap();

                if let Some(included_vec) = included.as_mut() {
                    if level % 2 == 0 {
                        included_vec[curr] = true;
                    }
                }

                for &neighbor in &adj[curr] {
                    if neighbor == parent {
                        continue;
                    }
                    queue.push_back((neighbor, curr));
                }
            }

            level += 1;
        }

        count
    }

    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;

        let mut adj1 = vec![vec![]; n];
        let mut adj2 = vec![vec![]; m];

        for edge in edges1 {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            adj1[u].push(v);
            adj1[v].push(u);
        }

        for edge in edges2 {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            adj2[u].push(v);
            adj2[v].push(u);
        }

        let even_count2 = Self::bfs(0, &adj2, None);
        let odd_count2 = m - even_count2;
        let best2 = even_count2.max(odd_count2);

        let mut included = vec![false; n];
        let even_count1 = Self::bfs(0, &adj1, Some(&mut included));
        let odd_count1 = n - even_count1;

        let mut res = vec![0; n];
        for i in 0..n {
            res[i] = if included[i] {
                (even_count1 + best2) as i32
            } else {
                (odd_count1 + best2) as i32
            };
        }

        res
    }
}
