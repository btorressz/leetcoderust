//1362. Closest Divisors

impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let sqrt_lmit = ((num + 2) as f64).sqrt() as i32;

        for i in (1..=sqrt_lmit).rev() {
            if (num + 1) % i == 0 {
                return vec![i, (num + 1) / i];
            }
            if (num + 2) % i == 0 {
                return vec![i, (num + 2) / i];
            }
        }

        vec![]
    }
}
