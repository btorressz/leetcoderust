struct CustomStack {
    stack: Vec<i32>,
    increments: Vec<i32>,
    max_size: usize,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        CustomStack {
            stack: Vec::new(),
            increments: Vec::new(),
            max_size: max_size as usize,
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push(x);
            self.increments.push(0); 
        }
    }

    fn pop(&mut self) -> i32 {
        if self.stack.is_empty() {
            return -1;
        }
        let idx = self.stack.len() - 1;
        let value = self.stack.pop().unwrap() + self.increments[idx];
        if idx > 0 {
            self.increments[idx - 1] += self.increments[idx]; 
        }
        self.increments.pop(); 
        value
    }

    fn increment(&mut self, k: i32, val: i32) {
        let k = k as usize;
        let size = self.stack.len();
        let idx = if k > size { size } else { k }; 
        if idx > 0 {
            self.increments[idx - 1] += val; 
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let mut obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */

