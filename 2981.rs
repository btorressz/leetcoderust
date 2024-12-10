//2981. Find Longest Special Substring That Occurs Thrice I

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let n = s.len();
        let mut substrings: Vec<String> = Vec::new();
        let mut mp: HashMap<String, i32> = HashMap::new();
        let mut max_length = 0;

        // Generate all substrings
        for i in 0..n {
            for j in i + 1..=n {
                substrings.push(s[i..j].to_string());
            }
        }

        // Count the frequency of substrings with a single unique character
        for substring in substrings {
            let unique_chars: HashSet<char> = substring.chars().collect();
            if unique_chars.len() == 1 {
                *mp.entry(substring).or_insert(0) += 1;
            }
        }

        // Find the maximum length of a substring that appears 3 or more times
        for (substring, count) in mp {
            if count >= 3 && substring.len() > max_length {
                max_length = substring.len();
            }
        }

        if max_length == 0 {
            -1
        } else {
            max_length as i32
        }
    }
}
