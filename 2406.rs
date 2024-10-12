use std::collections::BinaryHeap;

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        
        let mut min_heap: BinaryHeap<i32> = BinaryHeap::new();
        
        for interval in intervals {
            if let Some(&top) = min_heap.peek() {
                if interval[0] > -top {
                    min_heap.pop(); 
                }
            }
            min_heap.push(-interval[1]); 
        }
        
        min_heap.len() as i32
    }
}
