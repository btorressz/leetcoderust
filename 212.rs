




/* FIRST VERSION
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        // Store the found words
        let mut result = vec![];
        
        // DFS to check if we can find the word starting at (i, j)
        fn dfs(board: &Vec<Vec<char>>, word: &[char], i: usize, j: usize, visited: &mut Vec<Vec<bool>>, index: usize) -> bool {
            // If we've matched the whole word, return true
            if index == word.len() {
                return true;
            }

            // If out of bounds, already visited, or letters don't match, return false
            if i >= board.len() || j >= board[0].len() || visited[i][j] || board[i][j] != word[index] {
                return false;
            }

            // Mark this cell as visited
            visited[i][j] = true;

            // Try all 4 directions: right, down, left, up
            let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

            for (dx, dy) in directions {
                let x = i as isize + dx;
                let y = j as isize + dy;

                // Continue DFS if the next cell is within bounds and matches
                if x >= 0 && y >= 0 && dfs(board, word, x as usize, y as usize, visited, index + 1) {
                    return true;
                }
            }

            // Backtrack: unmark the cell as visited
            visited[i][j] = false;
            false
        }

        // Loop through each word to find it on the board
        for word in words.iter() {
            let word_chars: Vec<char> = word.chars().collect(); // Convert word to a vector of chars
            let mut found = false; // Flag to check if the word is found

            // Try to find the word starting from each cell on the board
            for i in 0..board.len() {
                for j in 0..board[0].len() {
                    let mut visited = vec![vec![false; board[0].len()]; board.len()]; // Reset visited cells
                    // Start DFS if the first letter matches
                    if dfs(&board, &word_chars, i, j, &mut visited, 0) {
                        result.push(word.clone()); // Add the word to the result
                        found = true;
                        break;
                    }
                }
                if found {
                    break; // Stop searching once the word is found
                }
            }
        }

        // Return the list of found words
        result
    }
} */

/*SECOND VERSION 
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut result = vec![];

        fn is_word_possible(board: &Vec<Vec<char>>, word: &str) -> bool {
            let mut char_count = std::collections::HashMap::new();
            for row in board {
                for &ch in row {
                    *char_count.entry(ch).or_insert(0) += 1;
                }
            }
            for ch in word.chars() {
                if let Some(count) = char_count.get_mut(&ch) {
                    if *count > 0 {
                        *count -= 1;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }

        fn dfs(
            board: &Vec<Vec<char>>,
            word: &[char],
            i: usize,
            j: usize,
            index: usize,
            visited: &mut Vec<Vec<bool>>,
        ) -> bool {
            if index == word.len() {
                return true;
            }

            if i >= board.len()
                || j >= board[0].len()
                || visited[i][j]
                || board[i][j] != word[index]
            {
                return false;
            }

            visited[i][j] = true;

            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for &(dx, dy) in &directions {
                let x = i as isize + dx;
                let y = j as isize + dy;
                if x >= 0
                    && y >= 0
                    && dfs(board, word, x as usize, y as usize, index + 1, visited)
                {
                    return true;
                }
            }

            visited[i][j] = false;
            false
        }

        let words: Vec<Vec<char>> = words
            .into_iter()
            .filter(|word| is_word_possible(&board, word)) 
            .map(|word| word.chars().collect())
            .collect();

        let mut visited = vec![vec![false; board[0].len()]; board.len()];

        for word in &words {
            let mut found = false;
            for i in 0..board.len() {
                for j in 0..board[0].len() {
                    if dfs(&board, word, i, j, 0, &mut visited) {
                        result.push(word.iter().collect());
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }

        result
    }
}

*/

/* Third TRy

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut result = vec![];

        // DFS function to search for a word
        fn dfs(
            board: &Vec<Vec<char>>,
            word: &[char],
            i: usize,
            j: usize,
            index: usize,
            visited: &mut Vec<Vec<bool>>,
        ) -> bool {
            if index == word.len() {
                // If all characters are matched
                return true;
            }

            // Bounds check and conditions
            if i >= board.len()
                || j >= board[0].len()
                || visited[i][j]
                || board[i][j] != word[index]
            {
                return false;
            }

            // Mark as visited
            visited[i][j] = true;

            // Explore all 4 directions
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for &(dx, dy) in directions.iter() {
                let x = i as isize + dx;
                let y = j as isize + dy;
                if x >= 0
                    && x < board.len() as isize
                    && y >= 0
                    && y < board[0].len() as isize
                {
                    if dfs(board, word, x as usize, y as usize, index + 1, visited) {
                        visited[i][j] = false; // Backtrack before returning
                        return true;
                    }
                }
            }

            // Backtrack
            visited[i][j] = false;
            false
        }

        // Loop through each word in the word list
        for word in words.iter() {
            let word_chars: Vec<char> = word.chars().collect();
            let mut found = false;

            // Check for the word starting from every cell
            for i in 0..board.len() {
                for j in 0..board[0].len() {
                    let mut visited = vec![vec![false; board[0].len()]; board.len()];
                    if dfs(&board, &word_chars, i, j, 0, &mut visited) {
                        result.push(word.clone());
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }

        result
    }
}
*/




