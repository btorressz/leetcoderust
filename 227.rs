impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut num = 0;
        let mut last_operator = '+';

        for (i, ch) in s.chars().enumerate() {
            if ch.is_digit(10) {
                num = num * 10 + (ch as u8 - b'0') as i32;
            }

            if !ch.is_digit(10) && ch != ' ' || i == s.len() - 1 {
                match last_operator {
                    '+' => stack.push(num),
                    '-' => stack.push(-num),
                    '*' => {
                        if let Some(last) = stack.pop() {
                            stack.push(last * num);
                        }
                    }
                    '/' => {
                        if let Some(last) = stack.pop() {
                            stack.push(last / num);
                        }
                    }
                    _ => {}
                }
                last_operator = ch;
                num = 0;
            }
        }

        stack.iter().sum()
    }
}
