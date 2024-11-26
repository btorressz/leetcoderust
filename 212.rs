
// I attempted this problem 4 times and will show my failed attempts below the successful one.
//SUCCESSFUL TRY
use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut words_set: HashSet<String> = HashSet::new();
        let mut prefix_set: HashSet<String> = HashSet::new();

        for word in &words {
            words_set.insert(word.clone());
            for i in 1..=word.len() {
                prefix_set.insert(word[..i].to_string());
            }
        }

        let mut result = HashSet::new();
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let m = board.len();
        let n = board[0].len();

        fn dfs(
            board: &Vec<Vec<char>>,
            i: usize,
            j: usize,
            visited: &mut Vec<Vec<bool>>,
            current_word: &mut String,
            words_set: &HashSet<String>,
            prefix_set: &HashSet<String>,
            result: &mut HashSet<String>,
        ) {
            if i >= board.len() || j >= board[0].len() || visited[i][j] {
                return;
            }

            current_word.push(board[i][j]);

            if !prefix_set.contains(current_word) {
                current_word.pop(); 
                return;
            }

            if words_set.contains(current_word) {
                result.insert(current_word.clone());
            }

            visited[i][j] = true;

            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for &(dx, dy) in &directions {
                let ni = i as isize + dx;
                let nj = j as isize + dy;
                if ni >= 0 && ni < board.len() as isize && nj >= 0 && nj < board[0].len() as isize {
                    dfs(
                        board,
                        ni as usize,
                        nj as usize,
                        visited,
                        current_word,
                        words_set,
                        prefix_set,
                        result,
                    );
                }
            }

            current_word.pop(); 
            visited[i][j] = false;
        }

        for i in 0..m {
            for j in 0..n {
                dfs(
                    &board,
                    i,
                    j,
                    &mut visited,
                    &mut String::new(),
                    &words_set,
                    &prefix_set,
                    &mut result,
                );
            }
        }

        result.into_iter().collect()
    }
}





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




