impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut in_degree = vec![0; n as usize];
        
        for edge in &edges {
            in_degree[edge[1] as usize] += 1;
        }
        
        let mut candidates = vec![];
        for (i, &degree) in in_degree.iter().enumerate() {
            if degree == 0 {
                candidates.push(i as i32);
            }
        }
        
        if candidates.len() == 1 {
            candidates[0]
        } else {
            -1
        }
    }
}
