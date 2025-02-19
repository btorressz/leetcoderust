//1415. The k-th Lexicographical String of All Happy Strings of Length n

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let total = if n == 1 { 3 } else { 3 * (1 << (n - 1)) };
        if total < k {
            return "".to_string();
        }
        if n == 1 {
            return ((b'a' + (k - 1) as u8) as char).to_string();
        }

        let mut result = String::new();
        let mut k = k - 1;

        for i in 0..n {
            let rest = 1 << (n - i - 1);
            let d = k / rest;
            let c = if i == 0 {
                (b'a' + d as u8) as char
            } else {
                match result.chars().last().unwrap() {
                    'a' => (b'b' + d as u8) as char,
                    'b' => if d == 0 { 'a' } else { 'c' },
                    _ => (b'a' + d as u8) as char,
                }
            };

            result.push(c);
            k %= rest;
        }

        result
    }
}
