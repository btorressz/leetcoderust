//1277. Count Square Submatrices with All Ones

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut counter = 0;

        let mut dp = vec![vec![0; n]; m];

        // Initialize first row and column
        for i in 0..m {
            dp[i][0] = matrix[i][0];
        }
        for j in 0..n {
            dp[0][j] = matrix[0][j];
        }

        // Fill the rest of the dp matrix
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = 1 + dp[i - 1][j]
                        .min(dp[i - 1][j - 1])
                        .min(dp[i][j - 1]);
                }
            }
        }

        // Count total squares
        for i in 0..m {
            for j in 0..n {
                counter += dp[i][j];
            }
        }

        counter
    }
}
