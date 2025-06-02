//3335. Total Characters in String After Transformations I

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let t = t as usize;

        let mut f = vec![vec![0; 26]; t + 1];

        for c in s.chars() {
            f[0][(c as u8 - b'a') as usize] += 1;
        }

        for i in 1..=t {
            f[i][0] = f[i - 1][25] % MOD;
            f[i][1] = (f[i - 1][0] + f[i - 1][25]) % MOD;
            for j in 2..26 {
                f[i][j] = f[i - 1][j - 1] % MOD;
            }
        }

        let mut res = 0;
        for j in 0..26 {
            res = (res + f[t][j]) % MOD;
        }

        res
    }
}
