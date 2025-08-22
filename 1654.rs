//1654. Minimum Jumps to Reach Home

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let limit = 2000 + 2 * b + 1;
        let mut visited = vec![(false, false); limit as usize]; // (forward visited, backward visited)

        // Mark forbidden positions
        for &f in &forbidden {
            if (f as usize) < visited.len() {
                visited[f as usize] = (true, true);
            }
        }

        let mut queue = VecDeque::new();
        queue.push_back((0, false)); // (position, came_from_backward)
        visited[0].0 = true;

        let mut steps = 0;

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let (pos, back) = queue.pop_front().unwrap();

                if pos == x {
                    return steps;
                }

                // Forward jump
                let forward = pos + a;
                if forward < limit
                    && !visited[forward as usize].0
                {
                    visited[forward as usize].0 = true;
                    queue.push_back((forward, false));
                }

                // Backward jump
                let backward = pos - b;
                if !back && backward >= 0 && !visited[backward as usize].1 {
                    visited[backward as usize].1 = true;
                    queue.push_back((backward, true));
                }
            }
            steps += 1;
        }

        -1
    }
}
