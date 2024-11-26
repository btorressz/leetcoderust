//371. Sum of Two Integers
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        Solution::get_sum(a ^ b, (a & b) << 1)

    }
}
