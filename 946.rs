//946. Validate Stack Sequences

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = vec![-1];  // Init stack with -1 to avoid empty stack
        let mut push_pointer = 0;
        let mut pop_pointer = 0;
        let n = pushed.len();

        while pop_pointer < n && push_pointer <= n {
            // Pop if top of stack matches next value to pop
            if stack.last() == Some(&popped[pop_pointer]) {
                stack.pop();
                pop_pointer += 1;
            } 
            // Push next value if stack top doesn't match
            else {
                if push_pointer >= n {
                    return false;  // No more values to push
                }
                stack.push(pushed[push_pointer]);
                push_pointer += 1;
            }
        }

        // Stack should only have the initial -1 value left
        stack.len() == 1
    }
}
