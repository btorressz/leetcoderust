//837. New 21 Game

use std::collections::HashMap;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 {
            return 1.0;
        }

        let mut window_sum = 0.0;

        // Calculate initial window sum for values from k to k + maxPts - 1
        for i in k..(k + max_pts) {
            if i <= n {
                window_sum += 1.0;
            }
        }

        let mut dp: HashMap<i32, f64> = HashMap::new();

        // Backward DP: from k-1 down to 0
        for i in (0..k).rev() {
            let prob = window_sum / max_pts as f64;
            dp.insert(i, prob);

            // Slide the window: add new dp[i], subtract old dp[i + maxPts]
            let remove = if i + max_pts <= n {
                *dp.get(&(i + max_pts)).unwrap_or(&1.0)
            } else {
                0.0
            };

            window_sum += prob - remove;
        }

        *dp.get(&0).unwrap_or(&0.0)
    }
}
