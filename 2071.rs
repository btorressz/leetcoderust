//2071. Maximum Number of Tasks You Can Assign

use std::collections::BTreeMap;

impl Solution {
    fn ok(tasks: &[i32], workers: &[i32], mut pills: i32, strength: i32, k: usize) -> bool {
        let mut st = BTreeMap::new();

        for &t in tasks.iter().take(k) {
            *st.entry(t).or_insert(0) += 1;
        }

        for i in (0..k).rev() {
            let w = workers[i];
            if let Some((&min_task, _)) = st.iter().next() {
                if w >= min_task {
                    Self::remove_from_multiset(&mut st, min_task);
                    continue;
                }
            }

            if pills > 0 {
                pills -= 1;
                let max_strength = w + strength;
                if let Some((&task, _)) = st.range(..=max_strength).next_back() {
                    Self::remove_from_multiset(&mut st, task);
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn remove_from_multiset(st: &mut BTreeMap<i32, i32>, val: i32) {
        if let Some(count) = st.get_mut(&val) {
            *count -= 1;
            if *count == 0 {
                st.remove(&val);
            }
        }
    }

    pub fn max_task_assign(mut tasks: Vec<i32>, mut workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        tasks.sort();
        workers.sort_by(|a, b| b.cmp(a)); 

        let mut l = 0;
        let mut r = tasks.len().min(workers.len());

        while l < r {
            let mid = (l + r + 1) / 2;
            if Self::ok(&tasks, &workers, pills, strength, mid) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }

        l as i32
    }
}
