impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        
        // sort the array to make it easier to find triplets
        nums.sort();
        
        let n = nums.len();
        
        for i in 0..n {
            // skip duplicates for the first number
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            
            let target = -nums[i];
            let mut left = i + 1;
            let mut right = n - 1;
            
            while left < right {
                let sum = nums[left] + nums[right];
                
                if sum == target {
                    // found a valid triplet
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    
                    // move pointers to skip duplicates
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    
                    left += 1;
                    right -= 1;
                } else if sum < target {
                    // increase sum by moving the left pointer
                    left += 1;
                } else {
                    // decrease sum by moving the right pointer
                    right -= 1;
                }
            }
        }
        
        result
    }
}
