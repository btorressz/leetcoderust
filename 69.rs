//69. Sqrt(x)
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 0 {
            return 0;
        }
        
        let mut left = 0;
        let mut right = x / 2 + 1;
        
        while left <= right {
            let mid = (right - left) / 2 + left;
            
            if mid * mid == x {
                return mid;
            } else if mid * mid < x {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        right
    }
}
