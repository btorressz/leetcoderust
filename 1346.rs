//1346. Check If N and Its Double Exist

use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut s = HashSet::new();
        
        for &x in arr.iter() {
            // Check if x * 2 or x / 2 exists in the set
            if s.contains(&(x * 2)) || (x % 2 == 0 && s.contains(&(x / 2))) {
                return true;
            }
            s.insert(x);
        }
        
        false
    }
}
