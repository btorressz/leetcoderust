//594. Longest Harmonious Subsequence

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort(); 
        let mut i = 0;
        let mut max_length = 0;

        for j in 0..nums.len() {
            while nums[j] - nums[i] > 1 {
                i += 1;
            }
            if nums[j] - nums[i] == 1 {
                max_length = max_length.max(j - i + 1);
            }
        }
        max_length as i32
    }
}
