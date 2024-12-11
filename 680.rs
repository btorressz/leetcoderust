//680. Valid Palindrome II

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        // Check if substring is palindrome
        fn check(s: &str, i: usize, j: usize) -> bool {
            let mut i = i;
            let mut j = j;
            while i < j {
                if s.as_bytes()[i] != s.as_bytes()[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }

        let mut i = 0;
        let mut j = s.len() - 1;

        // Try matching characters from both ends
        while i < j {
            if s.as_bytes()[i] != s.as_bytes()[j] {
                // Try skipping one character
                return check(&s, i, j - 1) || check(&s, i + 1, j);
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
