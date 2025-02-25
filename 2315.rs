//2315. Count Asterisks

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut res = 0; 
        let mut ins_bar = false;

        for c in s.chars() {
            if c == '|' {
                ins_bar = !ins_bar; // Toggle the inside/outside status
            } else if c == '*' && !ins_bar {
                res += 1; // Count '*' only if outside bars
            }
        }

        res
    }
}
