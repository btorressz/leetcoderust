//407. Trapping Rain Water II

use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        
        // Create a visited array to mark visited cells
        let mut vis = vec![vec![false; n]; m];
        
        // Min-heap (using Reverse to simulate a min-heap with BinaryHeap)
        let mut pq = BinaryHeap::new();
        
        // Add all boundary cells to the priority queue
        for i in 0..m {
            for j in 0..n {
                if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                    pq.push((std::cmp::Reverse(height_map[i][j]), i, j));
                    vis[i][j] = true;
                }
            }
        }

        // Directions: up, right, down, left
        let dirs: Vec<i32> = vec![-1, 0, 1, 0, -1]; // Explicitly set the type to i32
        
        // Initialize the result
        let mut res = 0;

        // While the priority queue is not empty
        while let Some((std::cmp::Reverse(h), i, j)) = pq.pop() {
            // Check all four directions
            for k in 0..4 {
                let x = i as i32 + dirs[k];
                let y = j as i32 + dirs[k + 1];
                // If the new position is within bounds and not visited
                if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 && !vis[x as usize][y as usize] {
                    // Calculate the trapped water
                    res += std::cmp::max(0, h - height_map[x as usize][y as usize]);
                    vis[x as usize][y as usize] = true;
                    pq.push((
                        std::cmp::Reverse(std::cmp::max(h, height_map[x as usize][y as usize])),
                        x as usize,
                        y as usize,
                    ));
                }
            }
        }
        
        res
    }
}
