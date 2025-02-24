//883. Projection Area of 3D Shapes

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut res = 0;

        for i in 0..n {
            let mut best_row = 0; // Largest value in row `i`
            let mut best_col = 0; // Largest value in column `i`
            
            for j in 0..n {
                if grid[i][j] > 0 {
                    res += 1; // Count nonzero cells for the top projection
                }
                best_row = best_row.max(grid[i][j]); // Get the max in the row
                best_col = best_col.max(grid[j][i]); // Get the max in the column
            }
            res += best_row + best_col;
        }

        res
    }
}
