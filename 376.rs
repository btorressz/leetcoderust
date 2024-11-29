//376. Wiggle Subsequence

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            // Return the length if there's 0 or 1 element
            return n as i32;
        }

        let mut prev_diff = 0; // Previous difference
        let mut count = 1; // Start with 1 element in the sequence

        for i in 1..n {
            let diff = nums[i] - nums[i - 1]; // Current difference
            if (diff > 0 && prev_diff <= 0) || (diff < 0 && prev_diff >= 0) {
                // Count valid wiggle
                count += 1;
                prev_diff = diff; // Update previous difference
            }
        }

        count // Return the result
    }
}
