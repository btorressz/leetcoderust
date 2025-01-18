//916. Word Subsets

use std::collections::HashMap;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        if words2.is_empty() {
            return words1;
        }

        // Build the lookup map for B (words2)
        let mut lookup: HashMap<char, i32> = HashMap::new();
        
        for word in &words2 {
            let mut tmp = HashMap::new();
            for c in word.chars() {
                *tmp.entry(c).or_insert(0) += 1;
            }
            for (key, &val) in &tmp {
                let count = lookup.entry(*key).or_insert(0);
                *count = std::cmp::max(*count, val);
            }
        }

        // Helper function to check if a word from A is a universal subset of B
        fn is_universal(tmp_a: &HashMap<char, i32>, tmp_b: &HashMap<char, i32>) -> bool {
            if tmp_a.len() < tmp_b.len() {
                return false;
            }
            for (key, &val) in tmp_b {
                if let Some(&a_val) = tmp_a.get(&key) {
                    if a_val < val {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }

        // Filter the words from A that are universal subsets of words in B
        words1.into_iter()
            .filter(|word| {
                let mut tmp_a = HashMap::new();
                for c in word.chars() {
                    *tmp_a.entry(c).or_insert(0) += 1;
                }
                is_universal(&tmp_a, &lookup)
            })
            .collect()
    }
}
