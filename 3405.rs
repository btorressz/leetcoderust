//3405. Count the Number of Arrays with K Matching Adjacent Elements

impl Solution {
    const MOD: i64 = 1_000_000_007;

    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let n = n as usize;
        let m = m as i64;
        let k = k as usize;

        let mut fact = vec![1i64; n + 1];
        let mut inv_fact = vec![1i64; n + 1];

        for i in 1..=n {
            fact[i] = fact[i - 1] * (i as i64) % Self::MOD;
        }

        inv_fact[n] = Self::mod_inv(fact[n]);
        for i in (1..=n).rev() {
            inv_fact[i - 1] = inv_fact[i] * (i as i64) % Self::MOD;
        }

        let run_ways = fact[n - 1] * inv_fact[n - k - 1] % Self::MOD * inv_fact[k] % Self::MOD;

        let assign_ways = m * Self::binary_exp(m - 1, (n - k - 1) as i64) % Self::MOD;

        (run_ways * assign_ways % Self::MOD) as i32
    }

    fn binary_exp(mut a: i64, mut b: i64) -> i64 {
        let mut res = 1i64;
        a %= Self::MOD;
        while b > 0 {
            if b & 1 == 1 {
                res = res * a % Self::MOD;
            }
            a = a * a % Self::MOD;
            b >>= 1;
        }
        res
    }

    fn mod_inv(val: i64) -> i64 {
        Self::binary_exp(val, Self::MOD - 2)
    }
}
