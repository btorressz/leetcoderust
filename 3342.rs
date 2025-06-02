//3342. Find Minimum Time to Reach Last Room II

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let n = move_time.len();
        let m = move_time[0].len();

        let mut result = vec![vec![i32::MAX; m]; n];
        let mut heap = BinaryHeap::new(); // min-heap using Reverse

        result[0][0] = 0;
        heap.push(Reverse((0, 0, 0))); // (currTime, i, j)

        while let Some(Reverse((curr_time, i, j))) = heap.pop() {
            if i == n - 1 && j == m - 1 {
                return curr_time;
            }

            for (di, dj) in directions.iter() {
                let ni = i as isize + di;
                let nj = j as isize + dj;

                if ni >= 0 && ni < n as isize && nj >= 0 && nj < m as isize {
                    let ni = ni as usize;
                    let nj = nj as usize;

                    let move_cost = if (ni + nj) % 2 == 0 { 2 } else { 1 };
                    let wait = (move_time[ni][nj] - curr_time).max(0);
                    let final_time = curr_time + wait + move_cost;

                    if final_time < result[ni][nj] {
                        result[ni][nj] = final_time;
                        heap.push(Reverse((final_time, ni, nj)));
                    }
                }
            }
        }

        -1
    }
}
