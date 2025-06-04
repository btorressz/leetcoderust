//2845. Count of Interesting Subarrays

use std::collections::HashMap;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let mut prefix_count = 0i32;
        let mut interesting_subarrays = 0i64;
        let mut mod_freq = HashMap::new();

        mod_freq.insert(0, 1i64);

        for &num in &nums {
            if num % modulo == k {
                prefix_count += 1;
            }

            prefix_count %= modulo;
            let key = (prefix_count - k + modulo) % modulo;

            if let Some(&count) = mod_freq.get(&key) {
                interesting_subarrays += count;
            }

            *mod_freq.entry(prefix_count).or_insert(0) += 1;
        }

        interesting_subarrays
    }
}
