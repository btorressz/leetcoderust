//645. Set Mismatch
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut freq = vec![0; n + 1]; 
        let mut duplicate = 0;
        let mut missing = 0;

        for &num in &nums {
            freq[num as usize] += 1;
        }

        for i in 1..=n {
            if freq[i] == 2 {
                duplicate = i as i32;
            } else if freq[i] == 0 {
                missing = i as i32;
            }
        }

        vec![duplicate, missing]
    }
}
