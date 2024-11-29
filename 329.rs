//329. Longest Increasing Path in a Matrix

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        // return 0 if the matrix is empty
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let m = matrix.len();
        let n = matrix[0].len();

        // dp table to store the longest path for each cell
        let mut dp = vec![vec![0; n]; m];

        // directions: down, up, right, left
        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        // dfs function to calculate the longest path starting from (i, j)
        fn dfs(i: usize, j: usize, matrix: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, directions: &Vec<(i32, i32)>) -> i32 {
            // if the longest path for this cell has been computed, return it
            if dp[i][j] != 0 {
                return dp[i][j];
            }

            // initialize the path length as 1 (the current cell itself)
            let mut max_length = 1;

            // explore the 4 possible directions
            for &(dx, dy) in directions.iter() {
                let ni = i as i32 + dx;
                let nj = j as i32 + dy;

                // check if the neighbor is within bounds and has a greater value
                if ni >= 0 && ni < matrix.len() as i32 && nj >= 0 && nj < matrix[0].len() as i32 {
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if matrix[ni][nj] > matrix[i][j] {
                        max_length = max_length.max(1 + dfs(ni, nj, matrix, dp, directions));
                    }
                }
            }

            dp[i][j] = max_length;
            dp[i][j]
        }

        // calculate the longest increasing path
        let mut longest_path = 0;
        for i in 0..m {
            for j in 0..n {
                longest_path = longest_path.max(dfs(i, j, &matrix, &mut dp, &directions));
            }
        }

        longest_path
    }
}
