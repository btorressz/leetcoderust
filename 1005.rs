//1005. Maximize Sum Of Array After K Negations

use std::collections::HashMap;

impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut frequency_map = HashMap::new();
        
        // Count occurrences of each number in nums
        for &num in &nums {
            *frequency_map.entry(num).or_insert(0) += 1;
        }

        // Negate numbers in the range [-100, 0)
        for x in -100..0 {
            if let Some(&count) = frequency_map.get(&x) {
                if count > 0 {
                    let m = count.min(k);
                    *frequency_map.entry(x).or_insert(0) -= m;
                    *frequency_map.entry(-x).or_insert(0) += m;
                    k -= m;
                    if k == 0 {
                        break;
                    }
                }
            }
        }

        // If k is odd and there are no 0's, negate the smallest positive number
        if k % 2 == 1 && frequency_map.get(&0).is_none() {
            for x in 1..=100 {
                if let Some(&count) = frequency_map.get(&x) {
                    if count > 0 {
                        *frequency_map.entry(x).or_insert(0) -= 1;
                        *frequency_map.entry(-x).or_insert(0) += 1;
                        break;
                    }
                }
            }
        }

        // Calculate the final sum
        frequency_map.iter().map(|(&num, &count)| num * count).sum()
    }
}
