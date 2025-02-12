// 2342. Max Sum of a Pair With Equal Sum of Digits

use std::collections::HashMap;


impl Solution {
    // Static method to calculate the digit sum
    fn digsum(mut n: i32) -> i32 {
        let mut r = 0;
        while n > 0 {
            r += n % 10;
            n /= 10;
        }
        r
    }

    // Method to calculate the maximum sum based on digit sums
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut s2i: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut result = -1;

        //  Populate the HashMap with digit sums and corresponding indices
        for (i, &n) in nums.iter().enumerate() {
            let s = Solution::digsum(n);
            s2i.entry(s).or_insert_with(Vec::new).push(i);
        }

        // Check each digit sum group
        for (_, indices) in s2i.iter_mut() {
            if indices.len() <= 1 {
                continue; // need at least two numbers for a valid sum
            }

            //  Sort the indices based on the corresponding values in the nums vector (descending)
            indices.sort_by(|&i1, &i2| nums[i2].cmp(&nums[i1]));

            //  Calculate the maximum sum of two largest numbers with the same digit sum
            result = result.max(nums[indices[0]] + nums[indices[1]]);
        }

        result
    }
}
