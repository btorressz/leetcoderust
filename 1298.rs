//1298. Maximum Candies You Can Get from Boxes

use std::collections::VecDeque;

impl Solution {
    pub fn max_candies(
        mut status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut q: VecDeque<usize> = VecDeque::new();
        for &i in &initial_boxes {
            q.push_back(i as usize);
        }

        let mut res = 0;
        let mut visited = vec![false; status.len()];

        while !q.is_empty() {
            let mut changed = false;
            let q_len = q.len();

            for _ in 0..q_len {
                let b = q.pop_front().unwrap();
                if status[b] == 1 && !visited[b] {
                    visited[b] = true;
                    changed = true;
                    res += candies[b];

                    for &i in &contained_boxes[b] {
                        q.push_back(i as usize);
                    }

                    for &i in &keys[b] {
                        status[i as usize] = 1;
                    }
                } else {
                    q.push_back(b);
                }
            }

            if !changed {
                return res;
            }
        }

        res
    }
}
