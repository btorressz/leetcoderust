//1920. Build Array from Permutation

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|&i| nums[i as usize]).collect()
    }
}
