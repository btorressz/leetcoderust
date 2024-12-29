//1639. Number of Ways to Form a Target String Given a Dictionary

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        
        let m = target.len(); 
        let n = words[0].len(); 
        
        let mut cnt = vec![vec![0; 26]; n];
        for word in &words {
            for (i, c) in word.chars().enumerate() {
                cnt[i][(c as u8 - b'a') as usize] += 1;
            }
        }

        let mut dp = vec![vec![-1; n + 1]; m + 1];
        
        fn dfs(
            i: usize,
            j: usize,
            m: usize,
            n: usize,
            target: &str,
            cnt: &Vec<Vec<i32>>,
            dp: &mut Vec<Vec<i32>>,
            mod_val: i32,
        ) -> i32 {
            if i == m {
                return 1; 
            }
            if j == n {
                return 0; 
            }
            if dp[i][j] != -1 {
                return dp[i][j]; 
            }

            let mut ans = dfs(i, j + 1, m, n, target, cnt, dp, mod_val) % mod_val;

            let target_char = target.as_bytes()[i] - b'a';
            if cnt[j][target_char as usize] > 0 {
                ans = (ans + (dfs(i + 1, j + 1, m, n, target, cnt, dp, mod_val) as i64
                    * cnt[j][target_char as usize] as i64 % mod_val as i64) as i32)
                    % mod_val;
            }

            dp[i][j] = ans;
            ans
        }

        dfs(0, 0, m, n, &target, &cnt, &mut dp, MOD)
    }
}
