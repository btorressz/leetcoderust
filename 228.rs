impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut i = 0; 
        
        while i < nums.len() {
            let start = nums[i];
            
            while i + 1 < nums.len() && nums[i + 1] == nums[i] + 1 {
                i += 1;
            }
            
            let end = nums[i];
            
            if start == end {
                result.push(format!("{}", start));
            } else {
                result.push(format!("{}->{}", start, end));
            }
            
            i += 1; 
        }
        
        result
    }
}
