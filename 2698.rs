//2698. Find the Punishment Number of an Integer

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        fn check(s: &str, i: usize, x: i32) -> bool {
            if i >= s.len() {
                return x == 0;
            }

            let mut y = 0;
            for j in i..s.len() {
                y = y * 10 + (s.chars().nth(j).unwrap() as i32 - '0' as i32);
                if y > x {
                    break;
                }
                if check(s, j + 1, x - y) {
                    return true;
                }
            }
            false
        }

        let mut res = 0;
        for i in 1..=n {
            let x = i * i;
            if check(&x.to_string(), 0, i) {
                res += x;
            }
        }
        res
    }
}
