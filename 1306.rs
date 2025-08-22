//1306. Jump Game III

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut visited = vec![false; arr.len()];
        let mut found = false;

        fn dfs(arr: &Vec<i32>, index: usize, visited: &mut Vec<bool>, found: &mut bool) {
            if *found {
                return;
            }

            let n = arr.len();
            visited[index] = true;

            let back = index as isize - arr[index] as isize;
            if back >= 0 {
                let b = back as usize;
                if !visited[b] {
                    if arr[b] == 0 {
                        *found = true;
                        return;
                    }
                    dfs(arr, b, visited, found);
                }
            }

            let forward = index + arr[index] as usize;
            if forward < n && !visited[forward] {
                if arr[forward] == 0 {
                    *found = true;
                    return;
                }
                dfs(arr, forward, visited, found);
            }
        }

        if arr[start as usize] == 0 {
            return true;
        }

        dfs(&arr, start as usize, &mut visited, &mut found);
        found
    }
}
