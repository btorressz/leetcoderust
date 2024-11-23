use std::collections::{HashSet};

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut guarded = HashSet::new(); // Tracks guarded cells
        let mut occupied = HashSet::new(); // Tracks guards and walls

        // Populate occupied set with guards and walls
        for guard in &guards {
            occupied.insert((guard[0], guard[1]));
        }
        for wall in &walls {
            occupied.insert((wall[0], wall[1]));
        }

        // Directions for north, south, east, west
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Simulate guard visibility
        for guard in &guards {
            let (start_row, start_col) = (guard[0], guard[1]);
            for &(dr, dc) in &directions {
                let (mut r, mut c) = (start_row, start_col);
                loop {
                    r += dr;
                    c += dc;

                    // Stop if out of bounds or blocked by a wall/guard
                    if r < 0 || r >= m || c < 0 || c >= n || occupied.contains(&(r, c)) {
                        break;
                    }

                    // Mark the cell as guarded
                    guarded.insert((r, c));
                }
            }
        }

        // Count unguarded cells
        let total_cells = (m * n) as usize;
        let occupied_or_guarded = guarded.len() + occupied.len();

        (total_cells - occupied_or_guarded) as i32
    }
}
