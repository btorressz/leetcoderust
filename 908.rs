//908. Smallest Range I

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            return 0; 
        }

        let max_num = *nums.iter().max().unwrap();
        let min_num = *nums.iter().min().unwrap();
        
        if k == 0 {
            return max_num - min_num;
        }
        
        let diff = max_num - min_num;
        
        if diff <= 2 * k {
            return 0; 
        }
        
        diff - 2 * k
    }
}
