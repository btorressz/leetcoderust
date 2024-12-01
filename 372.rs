// 372. Super Pow

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let mod_val = 1337;
        let mut ans = 1;
        let mut a = a % mod_val;

        let qpow = |mut a: i32, mut n: i32| -> i32 {
            let mut result = 1;
            while n > 0 {
                if n % 2 == 1 {
                    result = (result * a) % mod_val;
                }
                a = (a * a) % mod_val;
                n /= 2;
            }
            result
        };

        for &e in b.iter().rev() {
            ans = (ans * qpow(a, e)) % mod_val;
            a = qpow(a, 10);
        }

        ans
    }
}
