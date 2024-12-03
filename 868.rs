//868. Binary Gap

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        // Convert the integer to a binary string
        let bs = format!("{:b}", n);
        
        let mut max_dist = 0;
        let mut last = -1;
        
        // Iterate over the binary string and find the largest gap between '1's
        for (i, c) in bs.chars().enumerate() {
            if c == '1' {
                if last != -1 {
                    let dist = i as i32 - last;
                    if dist > max_dist {
                        max_dist = dist;
                    }
                }
                last = i as i32;
            }
        }
        
        max_dist
    }
}
