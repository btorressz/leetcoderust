//2537. Count the Number of Good Subarrays

use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut freq: HashMap<i32, i64> = HashMap::new();
        let mut equal_pairs: i64 = 0;
        let mut good_subarrays: i64 = 0;

        let n = nums.len();
        let mut left = 0;
        let mut right = 0;

        while left < n {
            while right < n && equal_pairs < k as i64 {
                let num = nums[right];
                let entry = freq.entry(num).or_insert(0);
                equal_pairs += *entry;
                *entry += 1;
                right += 1;
            }

            if equal_pairs >= k as i64 {
                good_subarrays += (n - right) as i64 + 1;
            }

            let left_num = nums[left];
            if let Some(entry) = freq.get_mut(&left_num) {
                *entry -= 1;
                equal_pairs -= *entry;
            }

            left += 1;
        }

        good_subarrays
    }
}
