//496. Next Greater Element I
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for &num in &nums1 {
            let mut found = false;
            let mut greater = -1;
            for &n in &nums2 {
                if n == num {
                    found = true;
                }
                if found && n > num {
                    greater = n;
                    break;
                }
            }
            result.push(greater);
        }
        result
    }
}
