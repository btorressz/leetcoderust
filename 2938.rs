
impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut steps = 0;
        let mut count_black = 0;

        for ch in s.chars() {
            if ch == '1' {
                count_black += 1;
            } else {
                steps += count_black;
            }
        }

        steps
    }
}
