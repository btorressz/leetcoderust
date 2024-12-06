//2262. Total Appeal of A String
//ATTEMPT TWO:SUCCESSFUL 
impl Solution {
    pub fn appeal_sum(s: String) -> i64 {
        let n = s.len();
        let bytes = s.as_bytes();  // Get the string as a byte slice
        let mut prev_letter_to_index = std::collections::HashMap::new();
        let mut dp = vec![0; n];  // dp[i] stores the total value of all substrings ending at index i
        let mut total_appeal = 0;  // This will store the sum of all appeals

        for i in 0..n {
            let current_char = bytes[i];  // Get the current character as a byte
            
            // Start with the value from dp[i-1] + (i + 1), which is assuming no repeats
            dp[i] = if i > 0 { dp[i - 1] + (i + 1) as i64 } else { 1 };
            
            // If the character has appeared before, subtract invalid substrings
            if let Some(&prev_index) = prev_letter_to_index.get(&current_char) {
                dp[i] -= (prev_index + 1) as i64;  // Subtract the "invalid" substrings
            }
            
            // Add dp[i] to total_appeal (sum of all appeals)
            total_appeal += dp[i];
            
            // Update the last seen index of the current character
            prev_letter_to_index.insert(current_char, i);
        }
        
        total_appeal
    }
}


/*ATTEMPT ONE: TIME LIMIT EXCEEDED 
impl Solution {
    pub fn appeal_sum(s: String) -> i64 {
        let n = s.len();
        let mut prev_letter_to_index = std::collections::HashMap::new();
        let mut dp = vec![1; n];  // dp[i] stores the total value of all substrings ending at index i
        
        // Initialize the first value of dp
        dp[0] = 1;
        prev_letter_to_index.insert(s.chars().nth(0).unwrap(), 0); // First character
        
        for i in 1..n {
            let current_char = s.chars().nth(i).unwrap(); // Get the current character
            
            dp[i] = dp[i - 1] + (i + 1) as i64;  // Assume no duplicates for now
            
            // If the character has appeared before, subtract invalid substrings
            if let Some(&prev_index) = prev_letter_to_index.get(&current_char) {
                dp[i] -= (prev_index + 1) as i64; // Subtract the "invalid" substrings
            }
            
            // Update the last seen index of the current character
            prev_letter_to_index.insert(current_char, i);
        }
        
        // Return the sum of all values in dp
        dp.iter().sum()
    }
}

*/
