//2596. Check Knight Tour Configuration

impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();

        // Check if the starting point is valid
        if grid[0][0] != 0 {
            return false;
        }

        // Map each number to its coordinates
        let mut coords_map = vec![vec![-1, -1]; (n * n) as usize];
        for r in 0..n {
            for c in 0..n {
                coords_map[grid[r][c] as usize] = vec![r as i32, c as i32];
            }
        }

        // Check if knight's movement is valid between consecutive numbers
        for value in 1..(n * n) {
            let prev = &coords_map[(value - 1) as usize];
            let curr = &coords_map[value as usize];
            
            let r_dist = (curr[0] - prev[0]).abs();
            let c_dist = (curr[1] - prev[1]).abs();
            
            // Knight's valid move: (1, 2) or (2, 1)
            if !( (r_dist == 1 && c_dist == 2) || (r_dist == 2 && c_dist == 1) ) {
                return false;
            }
        }

        true
    }
}
