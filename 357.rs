//357. Count Numbers with Unique Digits

use std::collections::HashMap;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let mut cache = HashMap::new();
        
        fn dfs(i: i32, mask: i32, lead: bool, cache: &mut HashMap<(i32, i32, bool), i32>) -> i32 {
            if i < 0 {
                return 1; // Valid number found
            }
            
            // Return cached result if already computed
            if let Some(&result) = cache.get(&(i, mask, lead)) {
                return result;
            }
            
            let mut ans = 0;
            
            for j in 0..10 {
                // Skip used digits
                if mask >> j & 1 != 0 {
                    continue;
                }
                
                // Allow leading zero only for the first digit
                if lead && j == 0 {
                    ans += dfs(i - 1, mask, true, cache);
                } else {
                    ans += dfs(i - 1, mask | 1 << j, false, cache);
                }
            }
            
            // Cache the result
            cache.insert((i, mask, lead), ans);
            ans
        }
        
        // Start DFS
        dfs(n - 1, 0, true, &mut cache)
    }
}
