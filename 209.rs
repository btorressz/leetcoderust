//209. Minimum Size Subarray Sum
//Solution 1
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut current_sum = 0;
        let mut min_length = i32::MAX;
        for end in 0..nums.len() {
            current_sum += nums[end];
            while current_sum >= target {
                min_length = min_length.min((end - start + 1) as i32);
                current_sum -= nums[start];
                start += 1;
            }
        }
        if min_length == i32::MAX {
            0
        } else {
            min_length
        }
    }
} //end of solution 1
