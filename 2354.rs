//2354. Number of Excellent Pairs

use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let uniq_nums: HashSet<i32> = nums.into_iter().collect();
        let mut bit_count_freq: HashMap<i32, i64> = HashMap::new();
        let mut res = 0;

        // Count the frequency of bit counts
        for &num in &uniq_nums {
            let bit_count = num.count_ones() as i32;
            *bit_count_freq.entry(bit_count).or_insert(0) += 1;
        }

        // Iterate over unique numbers and count valid pairs
        for &num in &uniq_nums {
            let t = num.count_ones() as i32;
            for (&bit_count, &freq) in &bit_count_freq {
                if t + bit_count >= k {
                    res += freq;
                }
            }
        }

        res
    }
}
