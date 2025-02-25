//685. Redundant Connection II

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parent = vec![0; n + 1];
        let mut root = (0..=n as i32).collect::<Vec<i32>>();
        let mut res1 = None;
        let mut res2 = None;

        for edge in &edges {
            let (u, v) = (edge[0], edge[1]);

            if parent[v as usize] > 0 {
                res1 = Some(vec![parent[v as usize], v]);
                res2 = Some(vec![u, v]);
                continue; 
            }
            parent[v as usize] = u;
        }

        for edge in &edges {
            let (u, v) = (edge[0], edge[1]);

            if res2 == Some(vec![u, v]) {
                continue;
            }

            let p = Self::find(u, &mut root);
            let q = Self::find(v, &mut root);

            if p == q {
                return res1.unwrap_or_else(|| edge.clone());
            }

            root[p as usize] = q;
        }

        res2.unwrap()
    }

    fn find(node: i32, root: &mut Vec<i32>) -> i32 {
        if root[node as usize] != node {
            root[node as usize] = Self::find(root[node as usize], root);
        }
        root[node as usize]
    }
}
