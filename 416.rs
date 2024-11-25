impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        // Calculate the total sum of the array
        let total_sum: i32 = nums.iter().sum();

        // If the total sum is odd, it cannot be split into two equal subsets
        if total_sum % 2 != 0 {
            return false;
        }

        // The target sum for each subset is half of the total sum
        let target = total_sum / 2;

        // Create a DP array where dp[i] indicates whether a subset sum of 'i' is possible
        let mut dp = vec![false; (target + 1) as usize];
        dp[0] = true; // A subset sum of 0 is always possible (by taking no elements)

        // Iterate over each number in the array
        for &num in &nums {
            // Traverse the DP array backwards from `target` to `num`
            // This ensures we do not overwrite results for the current number
            for i in (num..=target).rev() {
                // Update dp[i]: it's true if it was already true OR if dp[i - num] was true
                dp[i as usize] |= dp[(i - num) as usize];
            }
        }

        // The result is whether a subset with the target sum is possible
        dp[target as usize]
    }
}
