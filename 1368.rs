use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        
        // Directions: [up, right, left, down, up (for 1-based indexing of dirs)]
        let dirs = vec![
            (0, 0),  
            (0, 1),  // right
            (0, -1), // left
            (1, 0),  // down
            (-1, 0), // up
        ];
        
        let mut q = VecDeque::new();
        q.push_back((0, 0, 0)); // (i, j, cost)
        
        let mut vis = HashSet::new();
        
        while let Some((i, j, d)) = q.pop_front() {
            if vis.contains(&(i, j)) {
                continue;
            }
            vis.insert((i, j));
            
            if i == m - 1 && j == n - 1 {
                return d;
            }
            
            for k in 1..=4 {
                let (x, y) = (i as i32 + dirs[k as usize].0, j as i32 + dirs[k as usize].1);
                if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 {
                    let nx = x as usize;
                    let ny = y as usize;
                    if grid[i][j] == k {
                        q.push_front((nx, ny, d));
                    } else {
                        q.push_back((nx, ny, d + 1));
                    }
                }
            }
        }
        
        -1
    }
}
