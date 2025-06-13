//2401. Longest Nice Subarray

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut curr_sum = 0;
        let mut xor_sum = 0;
        let mut left = 0;
        let mut max_len = 0;

        for right in 0..n {
            let temp_right = nums[right];
            curr_sum += temp_right;
            xor_sum ^= temp_right;

            while curr_sum != xor_sum {
                let temp_left = nums[left];
                curr_sum -= temp_left;
                xor_sum ^= temp_left;
                left += 1;
            }

            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}
