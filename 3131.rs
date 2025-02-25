//3131. Find the Integer Added to Array I

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        nums2.iter().min().unwrap() - nums1.iter().min().unwrap()
    }
}
