//909. Snakes and Ladders

use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(mut board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let goal = (n * n) as i32;

        // helper function to map 1-based number to board coordinates
        fn get_pos(num: i32, n: usize) -> (usize, usize) {
            let num = num - 1;
            let y = (num / n as i32) as usize;
            let mut x = (num % n as i32) as usize;
            if y % 2 == 1 {
                x = n - 1 - x;
            }
            (n - 1 - y, x)
        }

        let mut queue = VecDeque::new();
        queue.push_back(1);
        let mut step = 0;
        board[n - 1][0] = 0; // mark visited

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let pos = queue.pop_front().unwrap();
                for offset in 1..=6 {
                    let next = pos + offset;
                    if next > goal {
                        continue;
                    }
                    if next == goal {
                        return step + 1;
                    }

                    let (y, x) = get_pos(next, n);
                    let cell = board[y][x];
                    if cell == -1 {
                        board[y][x] = 0; // mark visited
                        queue.push_back(next);
                    } else if cell > 0 {
                        if cell == goal {
                            return step + 1;
                        }
                        board[y][x] = 0; // mark visited
                        queue.push_back(cell);
                    }
                }
            }
            step += 1;
        }

        -1
    }
}
