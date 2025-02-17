//1079. Letter Tile Possibilities

use std::collections::HashSet;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        fn backtrack(path: &mut String, options: &mut Vec<char>, seen: &mut HashSet<String>, count: &mut i32) {
            if !path.is_empty() && seen.insert(path.clone()) {
                *count += 1;
            }

            for i in 0..options.len() {
                let add = options.remove(i); // Take the character out
                path.push(add); // Append it to path

                backtrack(path, options, seen, count);

                path.pop(); // Backtrack (remove last character)
                options.insert(i, add); // Restore the character
            }
        }

        let mut options: Vec<char> = tiles.chars().collect();
        let mut seen = HashSet::new();
        let mut count = 0;
        let mut path = String::new();

        backtrack(&mut path, &mut options, &mut seen, &mut count);
        count
    }
}
