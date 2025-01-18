//1400. Construct K Palindrome Strings

use std::collections::HashMap;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }

        let mut char_occurrences = HashMap::new();
        
        // Count occurrences of each character
        for c in s.chars() {
            *char_occurrences.entry(c).or_insert(0) += 1;
        }
        
        // Count how many characters have an odd frequency
        let odd_count = char_occurrences.values().filter(|&&v| v % 2 == 1).count();
        
        // Return true if the number of odd characters is <= k
        odd_count <= k as usize
    }
}
