//473. Matchsticks to Square
impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        
        // If the total sum is not divisible by 4, we cannot form a square
        if sum % 4 != 0 {
            return false;
        }
        
        let side_length = sum / 4;
        let n = matchsticks.len();
        let mut dp = vec![false; 1 << n]; // dp[state] indicates if this subset can contribute to forming a square
        let mut total = vec![0; 1 << n]; // total[state] stores the sum of matchsticks in this subset
        
        dp[0] = true; // Base case: empty subset is valid
        for state in 0..(1 << n) {
            if !dp[state] {
                continue;
            }
            for i in 0..n {
                // If the i-th matchstick is not in the current subset
                if (state & (1 << i)) == 0 {
                    let next_state = state | (1 << i);
                    if dp[next_state] {
                        continue;
                    }
                    // Add the i-th matchstick to the subset
                    let new_total = total[state] + matchsticks[i];
                    if new_total <= side_length {
                        dp[next_state] = true;
                        total[next_state] = new_total % side_length;
                    }
                }
            }
        }
        
        dp[(1 << n) - 1] // Check if the full set can form a square
    }
}
