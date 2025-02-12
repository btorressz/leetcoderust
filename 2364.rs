//2364. Count Number of Bad Pairs

use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut diff_count = HashMap::new();
        let mut bad_pairs = 0_i64;

        for (i, &n) in nums.iter().enumerate() {
            let d = i as i32 - n;
            bad_pairs += i as i64 - *diff_count.get(&d).unwrap_or(&0);
            *diff_count.entry(d).or_insert(0) += 1;
        }

        bad_pairs
    }
}
