//2673. Make Costs of Paths Equal in a Binary Tree

impl Solution {
    pub fn min_increments(n: i32, mut cost: Vec<i32>) -> i32 {
        let mut result = 0; 
        
        // loop from the last leaf node to the root (two nodes at a time)
        for i in (1..n).rev().step_by(2) {
            // increase result by the difference between two sibling nodes
            result += (cost[i as usize] - cost[(i - 1) as usize]).abs();
            
            // update the parent node with the max of the two siblings
            let parent_index = ((i - 1) / 2) as usize;
            cost[parent_index] += cost[i as usize].max(cost[(i - 1) as usize]);
        }
        
        result
    }
}
