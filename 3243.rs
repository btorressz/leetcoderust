//3243. Shortest Distance After Road Addition Queries I
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for i in 0..n - 1 {
            graph[i].push(i + 1);
        }

        let mut result = Vec::new();

        fn bfs(graph: &Vec<Vec<usize>>, n: usize) -> i32 {
            let mut visited = vec![false; n];
            let mut queue = VecDeque::new();
            queue.push_back((0, 0)); 
            visited[0] = true;

            while let Some((node, dist)) = queue.pop_front() {
                if node == n - 1 {
                    return dist; 
                }
                for &neighbor in &graph[node] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back((neighbor, dist + 1));
                    }
                }
            }

            -1 
        }

        for query in queries {
            let u = query[0] as usize;
            let v = query[1] as usize;

            graph[u].push(v);

            result.push(bfs(&graph, n));
        }

        result
    }
}
