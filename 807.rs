//807. Max Increase to Keep City Skyline
//// I used a greedy approach to maximize building heights while keeping the skyline unchanged.

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut maxrows = Vec::new();
        let mut maxcols = Vec::new();

        // Calculate the max building size for each row
        for row in &grid {
            maxrows.push(*row.iter().max().unwrap());
        }

        // Calculate the max building size for each column
        for i in 0..n {
            let mut col_max = i32::MIN;
            for j in 0..n {
                col_max = col_max.max(grid[j][i]);
            }
            maxcols.push(col_max);
        }

        let mut total = 0;
        // Traverse the grid, calculate the possible increase for each building
        for r in 0..n {
            for c in 0..n {
                total += std::cmp::max(0, std::cmp::min(maxrows[r], maxcols[c]) - grid[r][c]);
            }
        }

        total // Return the total increase in building heights
    }
}
