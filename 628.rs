impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;  
        nums.sort();  
        std::cmp::max(
            nums[nums.len() - 1] * nums[nums.len() - 2] * nums[nums.len() - 3], 
            nums[0] * nums[1] * nums[nums.len() - 1], 
        )
    }
}
