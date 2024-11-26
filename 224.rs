impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut num = 0;
        let mut result = 0;
        let mut sign = 1; 

        for ch in s.chars() {
            match ch {
                '0'..='9' => {
                    num = num * 10 + (ch as u8 - b'0') as i32;
                }
                '+' => {
                    result += sign * num;
                    num = 0;
                    sign = 1; 
                }
                '-' => {
                    result += sign * num;
                    num = 0;
                    sign = -1; 
                }
                '(' => {
                    stack.push(result);
                    stack.push(sign);
                    result = 0;
                    sign = 1;
                }
                ')' => {
                    result += sign * num; 
                    num = 0;
                    result *= stack.pop().unwrap(); 
                    result += stack.pop().unwrap(); 
                }
                _ => {} 
            }
        }

        result += sign * num;
        result
    }
}
