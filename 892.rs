//892. Surface Area of 3D Shapes

impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let n = grid.len();
        let mut res = 0;

        for r in 0..n {
            for c in 0..n {
                if grid[r][c] > 0 {
                    res += 2; 
                    
                    for &(dr, dc) in &directions {
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        let neighbor_value = if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                            grid[nr as usize][nc as usize]
                        } else {
                            0
                        };

                        res += (grid[r][c] - neighbor_value).max(0);
                    }
                }
            }
        }

        res
    }
}
