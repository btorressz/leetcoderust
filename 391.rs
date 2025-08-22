//391. Perfect Rectangle

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut area = 0;
        let mut min_x = rectangles[0][0];
        let mut min_y = rectangles[0][1];
        let mut max_x = rectangles[0][2];
        let mut max_y = rectangles[0][3];

        let mut count: HashMap<(i32, i32), i32> = HashMap::new();

        for r in &rectangles {
            let x1 = r[0];
            let y1 = r[1];
            let x2 = r[2];
            let y2 = r[3];

            // Add area of this rectangle
            area += (x2 - x1) * (y2 - y1);

            // Update bounding box
            min_x = min_x.min(x1);
            min_y = min_y.min(y1);
            max_x = max_x.max(x2);
            max_y = max_y.max(y2);

            // Count each corner
            *count.entry((x1, y1)).or_insert(0) += 1;
            *count.entry((x1, y2)).or_insert(0) += 1;
            *count.entry((x2, y1)).or_insert(0) += 1;
            *count.entry((x2, y2)).or_insert(0) += 1;
        }

        // The full rectangle area
        let expected_area = (max_x - min_x) * (max_y - min_y);
        if area != expected_area {
            return false;
        }

        // The 4 corner points should occur exactly once
        let corners = vec![
            (min_x, min_y),
            (min_x, max_y),
            (max_x, min_y),
            (max_x, max_y),
        ];

        for &corner in &corners {
            if count.remove(&corner) != Some(1) {
                return false;
            }
        }

        // All other internal points must appear 2 or 4 times
        for &val in count.values() {
            if val != 2 && val != 4 {
                return false;
            }
        }

        true
    }
}
