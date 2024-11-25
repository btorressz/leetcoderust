use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        // Convert the board into a single string representation
        let start = board.iter().flat_map(|row| row.iter()).map(|&x| x.to_string()).collect::<String>();
        let target = "123450".to_string();

        // Define the valid moves for each position of 0
        let neighbors = vec![
            vec![1, 3],       // 0 can move to 1 or 3
            vec![0, 2, 4],    // 1 can move to 0, 2, or 4
            vec![1, 5],       // 2 can move to 1 or 5
            vec![0, 4],       // 3 can move to 0 or 4
            vec![1, 3, 5],    // 4 can move to 1, 3, or 5
            vec![2, 4],       // 5 can move to 2 or 4
        ];

        // BFS setup
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((start.clone(), 0)); // (state, moves)
        visited.insert(start);

        while let Some((current, moves)) = queue.pop_front() {
            // Check if it reached the target state
            if current == target {
                return moves;
            }

            // Find the position of '0'
            let zero_pos = current.find('0').unwrap();

            // Generate all valid neighbors by swapping
            for &neighbor in &neighbors[zero_pos] {
                let mut next_state = current.clone().into_bytes();
                next_state.swap(zero_pos, neighbor);
                let next_state = String::from_utf8(next_state).unwrap();

                if visited.insert(next_state.clone()) {
                    queue.push_back((next_state, moves + 1));
                }
            }
        }

        // If it exhausts the queue without finding the target, return -1
        -1
    }
}
