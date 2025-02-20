//1980. Find Unique Binary String

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut result = String::new();

   for (i, num) in nums.iter().enumerate() {
            result.push(if num.chars().nth(i).unwrap() == '0' { '1' } else { '0' });
        }
        
        result
    }
}
