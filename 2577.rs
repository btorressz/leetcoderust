//2577. Minimum Time to Visit a Cell In a Grid


/* ATTEMPT ONE: WRONG ANSWER :(

use std::collections::VecDeque;

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        // If the first move is impossible
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }

        // Directions for moving up, down, left, right
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Deque for BFS
        let mut deque = VecDeque::new();
        deque.push_back((0, 0, 0)); // (time, row, col)

        // Visited matrix to avoid revisiting cells
        let mut visited = vec![vec![false; n]; m];
        visited[0][0] = true;

        while let Some((current_time, row, col)) = deque.pop_front() {
            // If reached the bottom-right cell
            if row == m - 1 && col == n - 1 {
                return current_time;
            }

            // Explore neighbors
            for &(dr, dc) in &directions {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                // Check bounds
                if new_row < 0 || new_row >= m as i32 || new_col < 0 || new_col >= n as i32 {
                    continue;
                }

                let new_row = new_row as usize;
                let new_col = new_col as usize;

                // Skip already visited cells
                if visited[new_row][new_col] {
                    continue;
                }

                // Calculate the earliest time to enter the neighbor
                if current_time + 1 >= grid[new_row][new_col] {
                    // Can enter immediately
                    visited[new_row][new_col] = true;
                    deque.push_front((current_time + 1, new_row, new_col));
                } else {
                    // Need to wait
                    let wait_time = grid[new_row][new_col] - (current_time + 1);
                    let next_time = current_time + 1 + if wait_time % 2 == 0 { wait_time } else { wait_time + 1 };
                    visited[new_row][new_col] = true;
                    deque.push_back((next_time, new_row, new_col));
                }
            }
        }

        // If exit the loop and haven't reached the bottom-right cell, return -1
        -1
    }
}



*/
