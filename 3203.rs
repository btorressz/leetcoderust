//3203. Find Minimum Diameter After Merging Two Trees

use std::collections::HashMap;

impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let d1 = Solution::tree_diameter(&edges1);
        let d2 = Solution::tree_diameter(&edges2);
        // Return the maximum of the individual diameters or the merged diameter
        std::cmp::max(d1, std::cmp::max(d2, (d1 + 1) / 2 + (d2 + 1) / 2 + 1))
    }

    fn tree_diameter(edges: &Vec<Vec<i32>>) -> i32 {
        let mut g = HashMap::new();
        
        // Build the graph using adjacency list
        for edge in edges {
            let (a, b) = (edge[0], edge[1]);
            g.entry(a).or_insert(vec![]).push(b);
            g.entry(b).or_insert(vec![]).push(a);
        }

        // Helper function for depth-first search
        fn dfs(i: i32, fa: i32, t: i32, g: &HashMap<i32, Vec<i32>>, ans: &mut i32, a: &mut i32) {
            // Safely access the adjacency list for node 'i'
            if let Some(neighbors) = g.get(&i) {
                for &j in neighbors {
                    if j != fa {
                        dfs(j, i, t + 1, g, ans, a);
                    }
                }
            }
            // Update the maximum distance and farthest node
            if *ans < t {
                *ans = t;
                *a = i;
            }
        }

        // Find the farthest node from an arbitrary starting node (node 0)
        let mut ans = 0;
        let mut a = 0;
        dfs(0, -1, 0, &g, &mut ans, &mut a);

        // Find the farthest node from node 'a', which gives the diameter
        ans = 0;
        dfs(a, -1, 0, &g, &mut ans, &mut a);

        ans
    }
}
