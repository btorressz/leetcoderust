use std::collections::HashMap;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut result = Vec::new();
        
        let mut p_count = [0; 26];
        let mut window_count = [0; 26];
        let p_len = p.len();
        let s_len = s.len();
        
        if s_len < p_len {
            return result;
        }
        
        // count frequencies of characters in p
        for &ch in p {
            p_count[(ch - b'a') as usize] += 1;
        }
        
        // sliding window on s
        for i in 0..s_len {
            // add current character to the window
            window_count[(s[i] - b'a') as usize] += 1;
            
            // remove the character that's sliding out of the window
            if i >= p_len {
                window_count[(s[i - p_len] - b'a') as usize] -= 1;
            }
            
            // compare window_count and p_count
            if window_count == p_count {
                result.push((i + 1 - p_len) as i32);
            }
        }
        
        result
    }
}
