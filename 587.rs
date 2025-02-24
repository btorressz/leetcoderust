//587. Erect the Fence

impl Solution {
    pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn cross(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> i32 {
            (b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0])
        }

        let n = trees.len();
        if n < 4 {
            return trees; // If there are less than 4 trees, they all belong to the hull.
        }

        // Sort points lexicographically (by x, then y)
        trees.sort_unstable();

        let mut hull: Vec<Vec<i32>> = Vec::new();

        // Construct lower hull
        for tree in &trees {
            while hull.len() > 1 && cross(&hull[hull.len() - 2], &hull[hull.len() - 1], tree) < 0 {
                hull.pop();
            }
            hull.push(tree.clone());
        }

        // Construct upper hull
        let lower_size = hull.len();
        for tree in trees.iter().rev() {
            while hull.len() > lower_size && cross(&hull[hull.len() - 2], &hull[hull.len() - 1], tree) < 0 {
                hull.pop();
            }
            hull.push(tree.clone());
        }

        // Remove duplicate last point
        hull.pop();

        // Remove duplicates to handle collinear edge cases
        hull.sort_unstable();
        hull.dedup();

        hull
    }
}

