//886. Possible Bipartition

use std::collections::VecDeque;

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for d in &dislikes {
            let u = (d[0] - 1) as usize;
            let v = (d[1] - 1) as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut color = vec![None; n];

        for i in 0..n {
            if color[i].is_none() {
                let mut queue = VecDeque::new();
                queue.push_back(i);
                color[i] = Some(true);
                while let Some(node) = queue.pop_front() {
                    for &neighbor in &graph[node] {
                        if color[neighbor].is_none() {
                            color[neighbor] = Some(!color[node].unwrap());
                            queue.push_back(neighbor);
                        } else if color[neighbor] == color[node] {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}
