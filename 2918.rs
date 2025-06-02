/*
attempt one wrong answer
impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let s1: i64 = nums1.iter().filter(|&&x| x != 0).map(|&x| x as i64).sum();
        let s2: i64 = nums2.iter().filter(|&&x| x != 0).map(|&x| x as i64).sum();

        let z1 = nums1.iter().filter(|&&x| x == 0).count() as i64;
        let z2 = nums2.iter().filter(|&&x| x == 0).count() as i64;

        let min_s1 = s1 + z1;
        let min_s2 = s2 + z2;

        if min_s1 > s2 + z2 || min_s2 > s1 + z1 {
            return -1;
        }

        min_s1.max(min_s2)
    }
}




*/
