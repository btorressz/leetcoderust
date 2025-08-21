//2348. Number of Zero-Filled Subarrays

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut counter: i64 = 0;
        let mut current: i64 = 0;

        for num in nums {
            if num == 0 {
                current += 1;
            } else {
                current = 0;
            }
            counter += current;
        }

        counter
    }
}
