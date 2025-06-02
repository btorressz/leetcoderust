//2359. Find Closest Node to Given Two Nodes

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        fn get_dist_arr(start: i32, edges: &Vec<i32>) -> Vec<i32> {
            let mut depth = vec![-1; edges.len()];
            let mut node = start;
            let mut dist = 0;

            while node != -1 && depth[node as usize] == -1 {
                depth[node as usize] = dist;
                dist += 1;
                node = edges[node as usize];
            }

            depth
        }

        let depth1 = get_dist_arr(node1, &edges);
        let depth2 = get_dist_arr(node2, &edges);

        let mut min_distance = i32::MAX;
        let mut res = -1;

        for i in 0..edges.len() {
            let d1 = depth1[i];
            let d2 = depth2[i];
            if d1 != -1 && d2 != -1 {
                let max_dist = d1.max(d2);
                if max_dist < min_distance {
                    min_distance = max_dist;
                    res = i as i32;
                }
            }
        }

        res
    }
}
