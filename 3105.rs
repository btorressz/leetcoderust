//3105. Longest Strictly Increasing or Strictly Decreasing Subarray
impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut direction = 0;
        let mut temp = 1;

        for i in 1..nums.len() {
            if direction != -1 && nums[i - 1] < nums[i] {
                direction = 1;
                temp += 1;
            } else if direction != 1 && nums[i - 1] > nums[i] {
                direction = -1;
                temp += 1;
            } else {
                direction *= -1;
                result = result.max(temp);
                temp = 1 + (nums[i - 1] != nums[i]) as i32;
            }
        }

        result.max(temp)
    }
}
