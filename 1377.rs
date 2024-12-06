// 1377. Frog Position After T Seconds

use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        // Create adjacency list graph
        let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges {
            g.entry(edge[0]).or_insert(Vec::new()).push(edge[1]);
            g.entry(edge[1]).or_insert(Vec::new()).push(edge[0]);
        }
        
        // Initialize queue with start node and probability
        let mut q: VecDeque<(i32, f64)> = VecDeque::new();
        q.push_back((1, 1.0));
        
        // Keep track of visited nodes
        let mut vis = vec![false; (n + 1) as usize];
        vis[1 as usize] = true;
        
        let mut time_left = t;
        
        while !q.is_empty() && time_left >= 0 {
            let level_size = q.len();
            
            for _ in 0..level_size {
                let (u, p) = q.pop_front().unwrap();
                
                // Count unvisited neighbors (subtract 1 for root node)
                let cnt = if u == 1 {
                    g.get(&u).map_or(0, |neighbors| neighbors.len())
                } else {
                    g.get(&u).map_or(0, |neighbors| 
                        neighbors.iter().filter(|&&v| !vis[v as usize]).count()
                    )
                };
                
                // If target is reached
                if u == target {
                    return if cnt as i32 * time_left == 0 { p } else { 0.0 };
                }
                
                // Explore neighbors
                if let Some(neighbors) = g.get(&u) {
                    for &v in neighbors {
                        if !vis[v as usize] {
                            vis[v as usize] = true;
                            q.push_back((v, p / cnt as f64));
                        }
                    }
                }
            }
            
            time_left -= 1;
        }
        
        0.0
    }
}
