// 403. Frog Jump

use std::collections::HashMap;

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();
        let mut pos = HashMap::new();

        // Store the position of each stone for O(1) lookup.
        for (i, &stone) in stones.iter().enumerate() {
            pos.insert(stone, i);
        }

        // Create a memoization map.
        let mut memo = HashMap::new();

        // Recursive dfs function with memoization.
        fn dfs(i: usize, k: i32, stones: &Vec<i32>, pos: &HashMap<i32, usize>, memo: &mut HashMap<(usize, i32), bool>) -> bool {
            if let Some(&cached_result) = memo.get(&(i, k)) {
                return cached_result;
            }
            
            if i == stones.len() - 1 {
                return true;
            }

            let mut result = false;
            for j in (k - 1)..=(k + 1) {
                if j > 0 {
                    if let Some(&next_idx) = pos.get(&(stones[i] + j)) {
                        if dfs(next_idx, j, stones, pos, memo) {
                            result = true;
                            break;
                        }
                    }
                }
            }

            memo.insert((i, k), result);
            result
        }

        dfs(0, 0, &stones, &pos, &mut memo)
    }
}
