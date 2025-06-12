use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, mut k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = nums.len();
        let mut arr = vec![];

        for (i, &num) in nums.iter().enumerate() {
            arr.push((i, Self::prime_factors(num), num));
        }

        let mut left = vec![-1; n];
        let mut right = vec![n as i32; n];

        let mut stk: Vec<usize> = Vec::new();
        for &(i, f, _) in &arr {
            while let Some(&top) = stk.last() {
                if arr[top].1 < f {
                    stk.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = stk.last() {
                left[i] = top as i32;
            }
            stk.push(i);
        }

        stk.clear();
        for i in (0..n).rev() {
            let f = arr[i].1;
            while let Some(&top) = stk.last() {
                if arr[top].1 <= f {
                    stk.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = stk.last() {
                right[i] = top as i32;
            }
            stk.push(i);
        }

        arr.sort_by(|a, b| b.2.cmp(&a.2)); 
        let mut ans = 1i64;

        for &(i, _, x) in &arr {
            let l = left[i] as i64;
            let r = right[i] as i64;
            let cnt = (i as i64 - l) * (r - i as i64);

            if cnt <= k as i64 {
                ans = ans * Self::qpow(x as i64, cnt as i64, MOD) % MOD;
                k -= cnt as i32;
            } else {
                ans = ans * Self::qpow(x as i64, k as i64, MOD) % MOD;
                break;
            }
        }

        ans as i32
    }

    fn prime_factors(mut n: i32) -> usize {
        let mut i = 2;
        let mut factors = HashSet::new();
        while i <= n / i {
            while n % i == 0 {
                factors.insert(i);
                n /= i;
            }
            i += 1;
        }
        if n > 1 {
            factors.insert(n);
        }
        factors.len()
    }

    fn qpow(mut a: i64, mut n: i64, m: i64) -> i64 {
        let mut res = 1;
        a %= m;
        while n > 0 {
            if n & 1 == 1 {
                res = res * a % m;
            }
            a = a * a % m;
            n >>= 1;
        }
        res
    }
}
