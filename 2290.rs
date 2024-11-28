//2290. Minimum Obstacle Removal to Reach Corner
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut dist_from_start = vec![vec![i32::MAX; n]; m];
        let mut dist_from_end = vec![vec![i32::MAX; n]; m];

        let mut start_queue = VecDeque::new();
        let mut end_queue = VecDeque::new();

        // Initialize both searches
        dist_from_start[0][0] = 0;
        start_queue.push_back((0, 0));
        dist_from_end[m - 1][n - 1] = 0;
        end_queue.push_back((m - 1, n - 1));

        // BFS from both ends
        while !start_queue.is_empty() || !end_queue.is_empty() {
            // Forward BFS
            if let Some((x, y)) = start_queue.pop_front() {
                for &(dx, dy) in &directions {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && ny >= 0 && nx < m as i32 && ny < n as i32 {
                        let nx = nx as usize;
                        let ny = ny as usize;

                        let new_cost = dist_from_start[x][y] + grid[nx][ny];
                        if new_cost < dist_from_start[nx][ny] {
                            dist_from_start[nx][ny] = new_cost;
                            start_queue.push_back((nx, ny));
                        }
                    }
                }
            }

            // Backward BFS
            if let Some((x, y)) = end_queue.pop_front() {
                for &(dx, dy) in &directions {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && ny >= 0 && nx < m as i32 && ny < n as i32 {
                        let nx = nx as usize;
                        let ny = ny as usize;

                        let new_cost = dist_from_end[x][y] + grid[nx][ny];
                        if new_cost < dist_from_end[nx][ny] {
                            dist_from_end[nx][ny] = new_cost;
                            end_queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }

        // Find the minimum where the two searches meet
        let mut result = i32::MAX;
        for i in 0..m {
            for j in 0..n {
                result = result.min(dist_from_start[i][j] + dist_from_end[i][j]);
            }
        }

        result
    }
}
