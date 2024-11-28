//477. Total Hamming Distance
impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                total += (nums[i] ^ nums[j]).count_ones() as i32;
            }
        }
        total
    }
}
