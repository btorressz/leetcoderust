//492 Construct the Rectangle
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        // Start with square root
        let mut w = (area as f64).sqrt() as i32;
        
        // Find divisor
        while area % w != 0 {
            w -= 1;
        }
        
        // Get length
        let l = area / w;
        
        // Return [length, width]
        vec![l, w]
    }
}
