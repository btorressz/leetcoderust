use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut pattern_count = HashMap::new();
        
        for row in &matrix {
            // Compute the signature: use the difference from the first value
            let pattern: Vec<i32> = row.iter().map(|&x| x ^ row[0]).collect();
            *pattern_count.entry(pattern).or_insert(0) += 1;
        }
        
        // Find the maximum count among all patterns
        pattern_count.values().copied().max().unwrap_or(0)
    }
}
