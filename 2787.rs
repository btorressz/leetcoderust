//attempt one: wrong answer 
/*impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = n as usize;
        let x = x as u32;

        let mut dp = vec![vec![0; n + 1]; n + 1];
        dp[0][0] = 1;

        for i in 1..=n {
            let k = (i as u32).pow(x) as usize;
            for j in 0..=n {
                dp[i][j] = dp[i - 1][j]; 
                if k <= j {
                    dp[i][j] = (dp[i][j] + dp[i - 1][j - k]) % MOD;
                }
            }
        }

        dp[n][n]
    }
}
*/

//Attempt two: correct use std::collections::HashMap;

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut memo = HashMap::new();

        // recursive function with memoization
        fn dfs(n: i32, i: i32, x: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
            if n == 0 {
                return 1;
            }
            if i.pow(x as u32) > n {
                return 0;
            }
            if let Some(&res) = memo.get(&(n, i)) {
                return res;
            }

            let inc = dfs(n - i.pow(x as u32), i + 1, x, memo) % MOD;
            let exc = dfs(n, i + 1, x, memo) % MOD;

            let res = (inc + exc) % MOD;
            memo.insert((n, i), res);
            res
        }

        dfs(n, 1, x, &mut memo)
    }
}

