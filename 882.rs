//attempt one: wrong answer 
/* use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

        for edge in &edges {
            let (u, v, cnt) = (edge[0], edge[1], edge[2] + 1);
            graph.entry(u).or_insert(Vec::new()).push((v, cnt));
            graph.entry(v).or_insert(Vec::new()).push((u, cnt));
        }

        if !graph.contains_key(&0) {
            return 1;
        }

        let mut pq = BinaryHeap::new();
        let mut dist = vec![i32::MAX; n]; 
        dist[0] = 0;
        pq.push(Reverse((0, 0))); 

        while let Some(Reverse((d, u))) = pq.pop() {
            if d > dist[u as usize] {
                continue;
            }
            if let Some(neighbors) = graph.get(&u) {
                for &(v, cnt) in neighbors {
                    let t = d + cnt;
                    if v < n as i32 && t < dist[v as usize] {
                        dist[v as usize] = t;
                        pq.push(Reverse((t, v)));
                    }
                }
            }
        }

        if dist.iter().all(|&d| d == i32::MAX) {
            return 1;
        }

        let mut res = dist.iter().filter(|&&d| d <= max_moves).count() as i32;

        for edge in &edges {
            let (u, v, cnt) = (edge[0], edge[1], edge[2]);
            let a = cnt.min(max_moves.saturating_sub(dist[u as usize]));
            let b = cnt.min(max_moves.saturating_sub(dist[v as usize]));
            res += cnt.min(a + b);
        }

        res
    }
}
*/
