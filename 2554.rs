//2554. Maximum Number of Integers to Choose From a Range I
impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned_set: std::collections::HashSet<i32> = banned.into_iter().collect();
        let mut count = 0;
        let mut sum = 0;
        
        // Iterate from 1 to n
        for i in 1..=n {
            if !banned_set.contains(&i) {
                if sum + i <= max_sum {
                    count += 1;
                    sum += i;
                } else {
                    return count;
                }
            }
        }
        
        count
    }
}
