//2131. Longest Palindrome by Concatenating Two Letter Words

use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut ret = 0;
        let mut record: HashMap<String, i32> = HashMap::new();

        for word in &words {
            let rev: String = word.chars().rev().collect();
            if let Some(count) = record.get_mut(&rev) {
                if *count > 0 {
                    *count -= 1;
                    ret += 4;
                    continue;
                }
            }
            *record.entry(word.clone()).or_insert(0) += 1;
        }

        for (k, v) in record.iter() {
            if *v > 0 && k.chars().nth(0) == k.chars().nth(1) {
                ret += 2;
                break;
            }
        }

        ret
    }
}
