//2116. Check if a Parentheses String Can Be Valid

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let n = s.len();
        if n % 2 != 0 {
            return false; 
        }
        
        let mut x = 0;
        let s_bytes = s.as_bytes();
        let locked_bytes = locked.as_bytes();
        
        for i in 0..n {
            if s_bytes[i] == b'(' || locked_bytes[i] == b'0' {
                x += 1;
            } else if x > 0 {
                x -= 1;
            } else {
                return false;
            }
        }

        x = 0;
        for i in (0..n).rev() {
            if s_bytes[i] == b')' || locked_bytes[i] == b'0' {
                x += 1;
            } else if x > 0 {
                x -= 1;
            } else {
                return false;
            }
        }

        true
    }
}
