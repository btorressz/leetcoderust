//3423. Maximum Difference Between Adjacent Elements in a Circular Array

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = i32::MIN;

        for i in 0..n - 1 {
            let diff = (nums[i] - nums[i + 1]).abs();
            res = res.max(diff);
        }

        res = res.max((nums[n - 1] - nums[0]).abs());

        res
    }
}
