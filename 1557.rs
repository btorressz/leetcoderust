//1557. Minimum Number of Vertices to Reach All Nodes

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut indegree = vec![0; n as usize];

        for edge in edges.iter() {
            if let Some(&to) = edge.get(1) {
                indegree[to as usize] += 1;
            }
        }

        let mut result = Vec::new();
        for i in 0..n as usize {
            if indegree[i] == 0 {
                result.push(i as i32);
            }
        }

        result
    }
}
