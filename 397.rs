//397. Integer Replacement
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut n = n as i64;
        let mut sts = 0;

        while n > 1 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                if n == 3 || n % 4 == 1 {
                    n -= 1;
                } else {
                    n += 1;
                }
            }
            sts += 1;
        }

        sts
    }
}
