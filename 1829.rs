impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let max_value = (1 << maximum_bit) - 1; 
        let mut current_xor = nums.iter().fold(0, |acc, &x| acc ^ x); 
        let mut result = Vec::new();

        for &num in nums.iter().rev() {
            result.push(max_value ^ current_xor); 
            current_xor ^= num; 
        }

        result
    }
}
