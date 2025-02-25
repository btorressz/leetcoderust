//2250. Count Number of Rectangles Containing Each Point

use std::collections::HashMap;

impl Solution {
    pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        let mut height_map: HashMap<i32, Vec<i32>> = HashMap::new();

        // Group rectangles by their y-coordinates and store x-values
        for rect in &rectangles {
            let (x, y) = (rect[0], rect[1]);
            height_map.entry(y).or_insert(vec![]).push(x);
        }

        // Sort x-values for each height
        for xs in height_map.values_mut() {
            xs.sort();
        }

        let mut res = Vec::new();

        // Process each point
        for point in points {
            let (x, y) = (point[0], point[1]);
            let mut count = 0;

            // Check all heights from y to 100 (inclusive)
            for h in y..=100 {
                if let Some(xs) = height_map.get(&h) {
                    let pos = xs.binary_search_by(|&val| val.cmp(&x)).unwrap_or_else(|i| i);
                    count += (xs.len() - pos) as i32;
                }
            }

            res.push(count);
        }

        res
    }
}

