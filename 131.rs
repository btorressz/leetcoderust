impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut current_partition = Vec::new();
        Self::backtrack(&s, 0, &mut current_partition, &mut result);
        result
    }

    fn backtrack(
        s: &String,
        start: usize,
        current_partition: &mut Vec<String>,
        result: &mut Vec<Vec<String>>
    ) {
        if start == s.len() {
            result.push(current_partition.clone()); 
            return;
        }

        for end in start..s.len() {
            if Self::is_palindrome(&s[start..=end]) {
                current_partition.push(s[start..=end].to_string()); 
                Self::backtrack(s, end + 1, current_partition, result); 
                current_partition.pop(); 
            }
        }
    }

    fn is_palindrome(substring: &str) -> bool {
        let chars: Vec<char> = substring.chars().collect();
        let len = chars.len();
        for i in 0..len / 2 {
            if chars[i] != chars[len - 1 - i] {
                return false;
            }
        }
        true
    }
}
