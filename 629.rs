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

