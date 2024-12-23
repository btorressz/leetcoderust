//2872. Maximum Number of K-Divisible Components

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let mut g = vec![vec![]; n as usize];
        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            g[a].push(b);
            g[b].push(a);
        }

        let mut ans = 0;

        fn dfs(
            node: usize,
            parent: i32,
            g: &Vec<Vec<usize>>,
            values: &Vec<i32>,
            k: i32,
            ans: &mut i32,
        ) -> i64 {
            let mut s = values[node] as i64; 

            for &neighbor in &g[node] {
                if neighbor != parent as usize {
                    s += dfs(neighbor, node as i32, g, values, k, ans);
                }
            }

            if s % (k as i64) == 0 {
                *ans += 1;
                return 0; 
            }

            s % (k as i64) 
        }

        dfs(0, -1, &g, &values, k, &mut ans);

        ans
    }
}
