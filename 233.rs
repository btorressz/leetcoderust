//233. Number of Digit One
impl Solution {
    // Count how many times the digit '1' appears in numbers from 1 to n
    pub fn count_digit_one(n: i32) -> i32 {
        (0..) // Start with powers of 10: 1, 10, 100, 
            .map(|factor| 10_i32.pow(factor as u32)) // Get 1, 10, 100, etc.
            .take_while(|&factor| factor <= n) // Stop when factor is bigger than n
            .map(|factor| {
                // Get parts of the number n
                let lower = n % factor; // The part after the current digit
                let current = (n / factor) % 10; // The current digit
                let higher = n / (factor * 10); // The part before the current digit

                match current {
                    0 => higher * factor, // No '1' here, just count higher digits
                    1 => higher * factor + lower + 1, // One '1' here, count it
                    _ => (higher + 1) * factor, // More than one '1' here, count it
                }
            })
            .sum() // Add up all the counts
    }
}
