//attempt one: wrong answer 
/*impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let k = k as usize;

        let mut f = vec![0; k + 1];
        f[0] = 1;
        let mut s = vec![0; k + 2];

        for i in 1..=n as usize {
            for j in 1..=k {
                f[j] = (s[j + 1] - s[j.saturating_sub(i - 1)]) % MOD;
            }
            for j in 1..=k + 1 {
                s[j] = (s[j - 1] + f[j - 1]) % MOD;
            }
        }

        f[k]
    }
}*/

//SUCCESSFUL ATTEMPT 
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let k = k as usize;
        let mut dp = vec![vec![0; k + 1]; (n + 1) as usize];

        dp[0][0] = 1; 

        for i in 1..=n as usize {
            let mut prefix_sum = vec![0; k + 2];

            for j in 0..=k {
                prefix_sum[j + 1] = (prefix_sum[j] + dp[i - 1][j]) % MOD;
            }

            for j in 0..=k {
                let left = if j >= i { prefix_sum[j + 1] - prefix_sum[j + 1 - i] } else { prefix_sum[j + 1] };
                dp[i][j] = (left + MOD) % MOD;
            }
        }

        dp[n as usize][k]
    }
}
