impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut counts = vec![('a', a), ('b', b), ('c', c)];
        let mut result = String::new();
        
        while !counts.is_empty() {
            counts.sort_by(|a, b| b.1.cmp(&a.1));
            
            let can_add = result.len() < 2 || 
                result.chars().rev().take(2).collect::<Vec<_>>() != vec![counts[0].0, counts[0].0];
            
            if counts[0].1 > 0 && can_add {
                result.push(counts[0].0);
                counts[0].1 -= 1;
            } else if counts.len() > 1 && counts[1].1 > 0 {
                result.push(counts[1].0);
                counts[1].1 -= 1;
            } else {
                break;
            }
            
            counts.retain(|&(_, count)| count > 0);
        }
        
        result
    }
}
