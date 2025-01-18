//3223. Minimum Length of String After Operations

use std::collections::HashMap;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut occurrences = HashMap::new();
        
        // Count occurrences of each character in the string
        for c in s.chars() {
            *occurrences.entry(c).or_insert(0) += 1;
        }
        
        // Sum up the values based on whether they're odd or even
        occurrences.values().map(|&x| if x % 2 == 1 { 1 } else { 2 }).sum()
    }
}
