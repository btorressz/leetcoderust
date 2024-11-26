impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        if left == right {
            return left;
        }
        Self::range_bitwise_and(left >> 1, right >> 1) << 1
    }
}
