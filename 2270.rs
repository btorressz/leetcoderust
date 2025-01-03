//2270. Number of Ways to Split Array

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let s: i64 = nums.iter().map(|&x| x as i64).sum(); // Total sum as i64
        let mut t: i64 = 0; // Running sum of the left part
        let mut val_spl = 0; // Counter for valid splits

        // Iterate through nums except the last element
        for &x in nums.iter().take(nums.len() - 1) {
            t += x as i64; // Cast each element to i64 before adding
            if t >= s - t {
                val_spl += 1;
            }
        }

        val_spl
    }
}
