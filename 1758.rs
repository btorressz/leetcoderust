impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut count_a = 0; 
        let mut count_b = 0; 

        for (i, c) in s.chars().enumerate() {
            let expected_char_a = if i % 2 == 0 { '0' } else { '1' };
            let expected_char_b = if i % 2 == 0 { '1' } else { '0' };

            if c != expected_char_a {
                count_a += 1;
            }
            if c != expected_char_b {
                count_b += 1;
            }
        }

        count_a.min(count_b)
    }
}
