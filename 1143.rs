impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        // Get the lengths of the two strings
        let m = text1.len();
        let n = text2.len();

        // Create a 2D DP array where dp[i][j] represents the length of the LCS of
        // the first i characters of text1 and the first j characters of text2
        let mut dp = vec![vec![0; n + 1]; m + 1];

        // Convert the strings to character arrays for easy indexing
        let chars1: Vec<char> = text1.chars().collect();
        let chars2: Vec<char> = text2.chars().collect();

        // Fill the DP table by comparing characters from text1 and text2
        for i in 1..=m {
            for j in 1..=n {
                // If the current characters match, extend the LCS by 1
                if chars1[i - 1] == chars2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    // Otherwise, take the maximum LCS by either excluding the current
                    // character from text1 or text2
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }

        // The bottom-right cell of the DP table contains the length of the LCS
        dp[m][n]
    }
}
