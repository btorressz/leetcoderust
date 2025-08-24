//844. Backspace String Compare

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        fn process(str: &str) -> String {
            let mut res = String::new();
            let mut skip = 0;
            for c in str.chars().rev() {
                if c == '#' {
                    skip += 1;
                } else if skip > 0 {
                    skip -= 1;
                } else {
                    res.push(c);
                }
            }
            res
        }

        process(&s) == process(&t)
    }
}
