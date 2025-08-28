

//attempt one wrong answer 

/*
impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();

        // Process diagonals from lower-left to main diagonal
        for k in (0..n - 1).rev() {
            let mut i = k;
            let mut j = 0;
            let mut diag = Vec::new();

            // Collect diagonal elements
            while i < n && j < n {
                diag.push(grid[i][j]);
                i += 1;
                j += 1;
            }

            // Sort and reassign
            diag.sort();
            let mut i = k;
            let mut j = 0;
            for val in diag {
                grid[i][j] = val;
                i += 1;
                j += 1;
            }
        }

        // Process diagonals from upper-right to below the main diagonal
        for k in (1..n - 1).rev() {
            let mut i = k;
            let mut j = n - 1;
            let mut diag = Vec::new();

            // Collect diagonal elements
            while i as isize >= 0 && j as isize >= 0 {
                diag.push(grid[i][j]);
                if i == 0 || j == 0 { break; } // prevent underflow
                i -= 1;
                j -= 1;
            }

            // Sort and reassign
            diag.sort();
            let mut i = k;
            let mut j = n - 1;
            for val in diag {
                grid[i][j] = val;
                if i == 0 || j == 0 { break; } // prevent underflow
                i -= 1;
                j -= 1;
            }
        }

        grid
    }
}











*/
