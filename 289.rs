//289. Game of Life
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),         (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        // Update board with encoded next state
        for i in 0..m {
            for j in 0..n {
                // Count live neighbors
                let mut live_neighbors = 0;
                for &(dr, dc) in &directions {
                    let nr = i as isize + dr;
                    let nc = j as isize + dc;
                    if nr >= 0 && nr < m as isize && nc >= 0 && nc < n as isize {
                        // use bit 0 to check the current state
                        if board[nr as usize][nc as usize] & 1 == 1 {
                            live_neighbors += 1;
                        }
                    }
                }

                // Apply the Game of Life rules (encode next state in bit 1)
                if board[i][j] & 1 == 1 { // current cell is live
                    if live_neighbors == 2 || live_neighbors == 3 {
                        board[i][j] |= 2; // Live → Live (set bit 1 to 1)
                    }
                } else { // Current cell is dead
                    if live_neighbors == 3 {
                        board[i][j] |= 2; // Dead → Live (set bit 1 to 1)
                    }
                }
            }
        }

        // Decode the next state
        for i in 0..m {
            for j in 0..n {
                board[i][j] >>= 1; // Shift right to get the next state
            }
        }
    }
}
