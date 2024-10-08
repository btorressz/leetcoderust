impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = Vec::new();
        
        for ch in s.chars() {
            stack.push(ch);
            if stack.len() >= 2 {
                let last = stack[stack.len() - 1];
                let second_last = stack[stack.len() - 2];
                
                if (second_last == 'A' && last == 'B') || (second_last == 'C' && last == 'D') {
                    stack.pop();
                    stack.pop();
                }
            }
        }
        
        stack.len() as i32
    }
}
