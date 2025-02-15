//485 Max Consecutive Ones
//Solution 1 : Linear Traversal	
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_count = 0;
        let mut current_count = 0;
        for num in nums {
            if num == 1 {
                current_count += 1;
                max_count = max_count.max(current_count); 
            } else {
                current_count = 0; 
            }
        }
        max_count
    }
} //end of solution 1

//Solution 2 : Grouping + Iterators
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.split(|&x| x == 0) 
            .map(|group| group.len() as i32) 
            .max() 
            .unwrap_or(0) 
    }
} //end of solution 2

