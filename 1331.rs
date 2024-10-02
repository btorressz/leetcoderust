use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut unique_elements: HashSet<i32> = HashSet::new();
        
        for &num in &arr {
            unique_elements.insert(num);
        }
        
        let mut sorted_unique: Vec<i32> = unique_elements.into_iter().collect();
        sorted_unique.sort();
        
        let mut rank_map: HashMap<i32, i32> = HashMap::new();
        for (rank, &num) in sorted_unique.iter().enumerate() {
            rank_map.insert(num, (rank + 1) as i32);
        }
        
        let mut ranked_arr: Vec<i32> = Vec::with_capacity(arr.len());
        for &num in &arr {
            ranked_arr.push(*rank_map.get(&num).unwrap());
        }
        
        ranked_arr
    }
}
