//390. Elimination Game

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut first_remaining = 1;
        let mut last_remaining = n;
        let mut rounds = 0;
        let mut step_size = 1;
        let mut remaining_count = n;
        
        while remaining_count > 1 {
            if rounds % 2 == 1 {
                last_remaining -= step_size;
                if remaining_count % 2 == 1 {
                    first_remaining += step_size;
                }
            } else {
                first_remaining += step_size;
                if remaining_count % 2 == 1 {
                    last_remaining -= step_size;
                }
            }
            
            remaining_count >>= 1;
            step_size <<= 1;
            rounds += 1;
        }
        
        first_remaining
    }
}
