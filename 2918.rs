//2918. Minimum Equal Sum of Two Arrays After Replacing Zeros

//Successful attempt 

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let sum1: i64 = nums1.iter().map(|&x| x as i64).sum::<i64>() + nums1.iter().filter(|&&x| x == 0).count() as i64;
        let sum2: i64 = nums2.iter().map(|&x| x as i64).sum::<i64>() + nums2.iter().filter(|&&x| x == 0).count() as i64;

        if sum1 > sum2 {
            return Solution::min_sum(nums2, nums1);
        }

        if sum1 == sum2 {
            return sum1;
        }

        if nums1.iter().all(|&x| x != 0) {
            return -1;
        }

        sum2
    }
}

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
