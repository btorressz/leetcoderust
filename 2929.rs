//2929. Distribute Candies Among Children II

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        if (n + 2) / 3 > limit {
            return 0;
        }

        let mut res: i64 = 0;
        for a in 0..=limit.min(n) {
            let rest = n - a;
            let min_b = if rest > limit { rest - limit } else { 0 };
            let max_b = rest.min(limit);

            if max_b >= min_b {
                res += (max_b - min_b + 1) as i64;
            }
        }

        res
    }
}
