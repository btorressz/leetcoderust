//2399. Check Distances Between Same Letters

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut positions = [[-1; 2]; 26]; // 
        
        let chars: Vec<char> = s.chars().collect();
        for (i, &ch) in chars.iter().enumerate() {
            let index = (ch as u8 - b'a') as usize;
            if positions[index][0] == -1 {
                positions[index][0] = i as i32;
            } else {
                positions[index][1] = i as i32;
            }
        }
        
        for i in 0..26 {
            if positions[i][1] != -1 {
                let actual_distance = positions[i][1] - positions[i][0] - 1;
                if actual_distance != distance[i] {
                    return false;
                }
            }
        }
        
        true
    }
}
