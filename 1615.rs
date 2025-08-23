//1615. Maximal Network Rank

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut degrees = vec![0; n];
        let mut connected = std::collections::HashSet::new();

        for road in &roads {
            let (u, v) = (road[0] as usize, road[1] as usize);
            degrees[u] += 1;
            degrees[v] += 1;
            connected.insert((u.min(v), u.max(v)));
        }

        let mut max_rank = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                let mut rank = degrees[i] + degrees[j];
                if connected.contains(&(i, j)) {
                    rank -= 1;
                }
                max_rank = max_rank.max(rank);
            }
        }

        max_rank
    }
}
