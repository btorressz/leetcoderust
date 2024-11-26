//209. Minimum Size Subarray Sum
//Solution 1 is using sliding window algorithm 
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

//Solution 2 
//For Solution 2 I tried out Binary Search.
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i];
        }
        let mut min_length = i32::MAX;
        for i in 0..n {
            let target_sum = target + prefix[i];
            if let Ok(j) = prefix.binary_search_by(|&x| x.cmp(&target_sum)) {
                min_length = min_length.min((j - i) as i32);
            } else if let Err(j) = prefix.binary_search(&target_sum) {
                if j <= n {
                    min_length = min_length.min((j - i) as i32);
                }
            }
        }
        if min_length == i32::MAX {
            0
        } else {
            min_length
        }
    }
} //end of solution 2 



