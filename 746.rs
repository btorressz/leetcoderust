impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; cost.len() + 1];
        
        dp[0] = cost[0];
        dp[1] = cost[1];
        
        for i in 2..=cost.len() {
            if i == cost.len() {
                dp[i] = dp[i - 1].min(dp[i - 2]);
            } else {
                dp[i] = dp[i - 1].min(dp[i - 2]) + cost[i];
            }
        }
        
        dp[cost.len()]
    }
}
