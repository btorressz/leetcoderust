//1971. Find if Path Exists in Graph
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut parent = (0..n).collect::<Vec<_>>();
        let mut rank = vec![0; n as usize];
        fn find(parent: &mut Vec<i32>, x: i32) -> i32 {
            if parent[x as usize] != x {
                parent[x as usize] = find(parent, parent[x as usize]); 
            }
            parent[x as usize]
        }
        fn union(parent: &mut Vec<i32>, rank: &mut Vec<i32>, x: i32, y: i32) {
            let root_x = find(parent, x);
            let root_y = find(parent, y);
            if root_x != root_y {
                if rank[root_x as usize] > rank[root_y as usize] {
                    parent[root_y as usize] = root_x;
                } else if rank[root_x as usize] < rank[root_y as usize] {
                    parent[root_x as usize] = root_y;
                } else {
                    parent[root_y as usize] = root_x;
                    rank[root_x as usize] += 1;
                }
            }
        }
        for edge in edges {
            union(&mut parent, &mut rank, edge[0], edge[1]);
        }

        find(&mut parent, source) == find(&mut parent, destination)
    }
}
