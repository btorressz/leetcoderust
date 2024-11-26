//684. Redundant Connection
//Solution 1: Using Union-Find (Disjoint Set Union) 
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        
        let mut parent = (0..=n as i32).collect::<Vec<i32>>();
        let mut rank = vec![1; n + 1];
        
        fn find(parent: &mut Vec<i32>, x: i32) -> i32 {
            if parent[x as usize] != x {
                parent[x as usize] = find(parent, parent[x as usize]); 
            }
            parent[x as usize]
        }
        
        fn union(parent: &mut Vec<i32>, rank: &mut Vec<i32>, x: i32, y: i32) -> bool {
            let root_x = find(parent, x);
            let root_y = find(parent, y);
            
            if root_x == root_y {
                return false; 
            }
            
            if rank[root_x as usize] > rank[root_y as usize] {
                parent[root_y as usize] = root_x;
            } else if rank[root_x as usize] < rank[root_y as usize] {
                parent[root_x as usize] = root_y;
            } else {
                parent[root_y as usize] = root_x;
                rank[root_x as usize] += 1;
            }
            true
        }
        
        for edge in edges {
            let u = edge[0];
            let v = edge[1];
            
            if !union(&mut parent, &mut rank, u, v) {
                return vec![u, v]; 
            }
        }
        
        vec![] 
    }
}
