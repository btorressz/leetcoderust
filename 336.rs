//336. Palindrome Pairs

use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut d: HashMap<String, usize> = HashMap::new();
        let mut val = Vec::new();

        // Build the dictionary: word -> index
        for (i, w) in words.iter().enumerate() {
            d.insert(w.clone(), i);
        }

        // Iterate through each word and its index
        for (i, w) in words.iter().enumerate() {
            let len = w.len();
            for j in 0..=len {
                let (a, b) = w.split_at(j); // Split into two substrings
                
                let ra: String = a.chars().rev().collect(); // Reverse a
                let rb: String = b.chars().rev().collect(); // Reverse b
                
                // Case 1: Check if a reversed exists and b is a palindrome
                if let Some(&idx) = d.get(&ra) {
                    if idx != i && b == rb {
                        val.push(vec![i as i32, idx as i32]);
                    }
                }
                
                // Case 2: Check if b reversed exists and a is a palindrome
                if j > 0 {
                    if let Some(&idx) = d.get(&rb) {
                        if idx != i && a == ra {
                            val.push(vec![idx as i32, i as i32]);
                        }
                    }
                }
            }
        }

        val
    }
}
