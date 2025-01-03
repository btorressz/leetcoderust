//1422. Maximum Score After Splitting a String

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect(); 
        let total_ones = s.iter().filter(|&&c| c == '1').count(); 
        
        let mut max_score = 0;
        let mut left_zeros = 0;
        let mut right_ones = total_ones;
        
        for i in 0..s.len() - 1 {
            if s[i] == '0' {
                left_zeros += 1;
            } else {
                right_ones -= 1;
            }
            max_score = max_score.max(left_zeros + right_ones);
        }
        
        max_score as i32
    }
}
