use std::collections::BinaryHeap;
use std::cmp::Reverse;

//3341. Find Minimum Time to Reach Last Room I

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let n = move_time.len();
        let m = move_time[0].len();

        let mut result = vec![vec![i32::MAX; m]; n];

        let mut heap = BinaryHeap::new();
        result[0][0] = 0;
        heap.push(Reverse((0, 0, 0))); // (time, row, col)

        while let Some(Reverse((curr_time, i, j))) = heap.pop() {
            if i == n - 1 && j == m - 1 {
                return curr_time;
            }

            for (di, dj) in directions.iter() {
                let new_i = i as isize + di;
                let new_j = j as isize + dj;

                if new_i >= 0 && new_i < n as isize && new_j >= 0 && new_j < m as isize {
                    let ni = new_i as usize;
                    let nj = new_j as usize;
                    let wait = (move_time[ni][nj] - curr_time).max(0);
                    let final_time = curr_time + wait + 1;

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
