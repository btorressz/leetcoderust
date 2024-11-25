use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // convert the input string into bytes for easier indexing
        let s = s.as_bytes();
        let mut seen = HashSet::new(); // to track characters in the current window
        let mut max_length = 0; // to store the maximum length found
        let mut left = 0; // left pointer for the sliding window
        
        // iterate through the string with a right pointer
        for right in 0..s.len() {
            // if the character at 'right' is already in the set, remove characters
            // from the left of the window until it's not in the set
            while seen.contains(&s[right]) {
                seen.remove(&s[left]);
                left += 1;
            }
            // add the current character to the set
            seen.insert(s[right]);
            // update the maximum length if needed
            max_length = max_length.max(right - left + 1);
        }
        
        max_length as i32
    }
}
