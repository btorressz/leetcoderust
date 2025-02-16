//attempt 1 
/*impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        fn dfs(u: usize, n: usize, path: &mut Vec<i32>, cnt: &mut Vec<i32>) -> bool {
            if u == n * 2 {
                return true;
            }
            if path[u] != 0 {
                return dfs(u + 1, n, path, cnt);
            }
            
            for i in (2..=n).rev() {
                if cnt[i] > 0 && u + i < n * 2 && path[u + i] == 0 {
                    cnt[i] -= 1;
                    path[u] = i as i32;
                    path[u + i] = i as i32;
                    if dfs(u + 1, n, path, cnt) {
                        return true;
                    }
                    path[u] = 0;
                    path[u + i] = 0;
                    cnt[i] += 1;
                }
            }
            
            if cnt[1] > 0 {
                cnt[1] -= 1;
                path[u] = 1;
                if dfs(u + 1, n, path, cnt) {
                    return true;
                }
                path[u] = 0;
                cnt[1] += 1;
            }
            
            false
        }

        let mut path = vec![0; (n * 2) as usize];
        let mut cnt = vec![2; (n * 2) as usize];
        cnt[1] = 1;
        
        dfs(1, n as usize, &mut path, &mut cnt);
        path[1..].to_vec()
    }
}*/
