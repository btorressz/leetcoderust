//2684. Maximum Number of Moves in a Grid

use std::collections::HashMap;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        // Memoization using a HashMap
        let mut memo = HashMap::new();

        // DFS function to find max moves from a given cell
        fn dfs(r: usize, c: usize, m: usize, n: usize, grid: &Vec<Vec<i32>>, memo: &mut HashMap<(usize, usize), i32>) -> i32 {
            // If result is already cached, return it
            if let Some(&cached_result) = memo.get(&(r, c)) {
                return cached_result;
            }

            let mut curr_result = 0;
            // Try all 3 possible directions (up-right, right, down-right)
            for (dr, dc) in [(-1, 1), (0, 1), (1, 1)] {
                let new_r = r as isize + dr;
                let new_c = c as isize + dc;
                if new_r >= 0 && new_r < m as isize && new_c >= 0 && new_c < n as isize {
                    let new_r = new_r as usize;
                    let new_c = new_c as usize;
                    if grid[new_r][new_c] > grid[r][c] {
                        // Continue DFS if the new cell is valid
                        curr_result = curr_result.max(1 + dfs(new_r, new_c, m, n, grid, memo));
                    }
                }
            }

            // Memoize the result before returning
            memo.insert((r, c), curr_result);
            curr_result
        }

        let mut result = 0;
        // Try every cell in the first column
        for i in 0..m {
            result = result.max(dfs(i, 0, m, n, &grid, &mut memo));
        }

        result
    }
}
