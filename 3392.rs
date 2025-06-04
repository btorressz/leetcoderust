//3392. Count Subarrays of Length Three With a Condition

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = 0;

        for i in 1..n - 1 {
            if nums[i] == (nums[i - 1] + nums[i + 1]) * 2 {
                count += 1;
            }
        }

        count
    }
}
