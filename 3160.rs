// 3160. Find the Number of Distinct Colors Among the Balls

use std::collections::HashMap;

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut color_count = HashMap::new(); // Tracks the number of balls with each color
        let mut index_color = HashMap::new(); // Maps ball index to its color
        let mut distinct_count = 0; // Number of distinct colors
        let mut result = Vec::new();

        for query in queries {
            let index = query[0];
            let color = query[1];

            if let Some(&old_color) = index_color.get(&index) {
                // Decrease count of the previous color
                if let Some(count) = color_count.get_mut(&old_color) {
                    *count -= 1;
                    if *count == 0 {
                        color_count.remove(&old_color);
                        distinct_count -= 1;
                    }
                }
            }

            // Assign new color to the ball
            index_color.insert(index, color);

            // Increase count of the new color
            let count = color_count.entry(color).or_insert(0);
            if *count == 0 {
                distinct_count += 1;
            }
            *count += 1;

            result.push(distinct_count);
        }

        result
    }
}
