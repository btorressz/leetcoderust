//2349. Design a Number Container System

use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
  index_map: HashMap<i32, i32>, 
  number_map: HashMap<i32, BTreeSet<i32>>, 
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
 
impl NumberContainers {
    fn new() -> Self {
        Self {
            index_map: HashMap::new(),
            number_map: HashMap::new(),
        }
    }
    
    fn change(&mut self, index: i32, number: i32) {
        if let Some(&old_number) = self.index_map.get(&index) {
            if let Some(set) = self.number_map.get_mut(&old_number) {
                set.remove(&index);
                if set.is_empty() {
                    self.number_map.remove(&old_number);
                }
            }
        }
        
        self.index_map.insert(index, number);
        self.number_map.entry(number).or_insert_with(BTreeSet::new).insert(index);
    }
    
    fn find(&self, number: i32) -> i32 {
        self.number_map.get(&number)
            .and_then(|set| set.iter().next().cloned())
            .unwrap_or(-1)
    }
}

       

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */
