//attempt two correct answer 
//3446. Sort Matrix by Diagonals
//At first, I incorrectly applied the same sorting logic to all diagonals in the matrix, without distinguishing between the bottom-left 
//and top-right triangles. This caused the output to fail because some diagonals needed to be sorted in descending order, while others needed ascending. I fixed it by identifying which triangle each 
//diagonal belonged to: I sorted the bottom-left diagonals (including the main diagonal) 
//in non-increasing order and the top-right diagonals in non-decreasing order. This ensured the output matched the problem’s directional sorting constraints.

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();

        // Helper: sort and reassign diagonal
        fn process_diagonal(
            mut i: usize,
            mut j: usize,
            n: usize,
            grid: &mut Vec<Vec<i32>>,
            reverse: bool,
        ) {
            let mut diag = Vec::new();
            let (mut x, mut y) = (i, j);

            while x < n && y < n {
                diag.push(grid[x][y]);
                x += 1;
                y += 1;
            }

            if reverse {
                diag.sort_by(|a, b| b.cmp(a)); // descending
            } else {
                diag.sort(); // ascending
            }

            let (mut x, mut y) = (i, j);
            for val in diag {
                grid[x][y] = val;
                x += 1;
                y += 1;
            }
        }

        // Bottom-left triangle (including main diagonal) → sort in descending
        for i in (0..n).rev() {
            process_diagonal(i, 0, n, &mut grid, true);
        }

        // Top-right triangle → sort in ascending (exclude main diagonal which is already processed)
        for j in 1..n {
            process_diagonal(0, j, n, &mut grid, false);
        }

        grid
    }
}


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
