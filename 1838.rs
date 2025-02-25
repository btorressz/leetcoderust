//1838. Frequency of the Most Frequent Element

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort(); 

        let mut max_length = 0;
        let mut current_sum = 0_i64;
        let mut left_window = 0;
        let k = k as i64; 

        for right_window in 0..nums.len() {
            let right_num = nums[right_window] as i64;
            current_sum += right_num;

            let current_length = (right_window - left_window + 1) as i64;

            if current_sum + k >= right_num * current_length {
                max_length = max_length.max(current_length as i32);
            } else {
                current_sum -= nums[left_window] as i64;
                left_window += 1;
            }
        }

        max_length
    }
}
