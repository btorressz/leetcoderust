//436. Find Right Interval

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut start_to_index = std::collections::HashMap::new();
        let mut start_values: Vec<i32> = Vec::new();

        // Store start points and create a hashmap of indexes for start points
        for (i, interval) in intervals.iter().enumerate() {
            let start = interval[0];
            start_values.push(start);
            start_to_index.insert(start, i as i32);
        }

        // Sort the start values to use binary search
        start_values.sort();
        let n = intervals.len();
        let mut result = Vec::new();

        // For each end value, do a binary search on sorted start values
        for interval in intervals {
            let end = interval[1];

            // Binary search for the smallest start point >= end
            match start_values.binary_search(&end) {
                Ok(pos) => result.push(start_to_index[&start_values[pos]]), // Found the right interval
                Err(pos) => {
                    if pos == n {
                        result.push(-1); // No valid start found
                    } else {
                        result.push(start_to_index[&start_values[pos]]);
                    }
                }
            }
        }

        result
    }
}
