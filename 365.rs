//365. Water and Jug Problem

use std::collections::HashSet;

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
        let mut vis = HashSet::new();
        
        fn dfs(i: i32, j: i32, x: i32, y: i32, target: i32, vis: &mut HashSet<(i32, i32)>) -> bool {
            // If we've seen this state before, skip it
            if vis.contains(&(i, j)) {
                return false;
            }
            vis.insert((i, j));  // Mark state as visited
            
            // If either jug has the target amount, return true
            if i == target || j == target || i + j == target {
                return true;
            }
            
            // Try all possible moves
            if dfs(x, j, x, y, target, vis) { return true; }  // Fill first jug
            if dfs(i, y, x, y, target, vis) { return true; }  // Fill second jug
            if dfs(0, j, x, y, target, vis) { return true; }  // Empty first jug
            if dfs(i, 0, x, y, target, vis) { return true; }  // Empty second jug
            
            // Pour from one jug to the other
            let a = std::cmp::min(i, y - j);  // Max we can pour into the second jug
            let b = std::cmp::min(j, x - i);  // Max we can pour into the first jug
            if dfs(i - a, j + a, x, y, target, vis) { return true; }  // Pour from first to second
            if dfs(i + b, j - b, x, y, target, vis) { return true; }  // Pour from second to first
            
            false  // No solution found
        }
        
        // Start from both jugs empty
        dfs(0, 0, x, y, target, &mut vis)
    }
}
