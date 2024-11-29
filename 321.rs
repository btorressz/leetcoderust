//321. Create Maximum Number

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        fn max_subsequence(nums: &Vec<i32>, k: usize) -> Vec<i32> {
            let mut stack = Vec::new();
            let mut drop = nums.len() - k; 
            for &num in nums.iter() {
                while !stack.is_empty() && *stack.last().unwrap() < num && drop > 0 {
                    stack.pop();
                    drop -= 1;
                }
                stack.push(num);
            }
            stack.truncate(k); 
            stack
        }

        fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
            let mut result = Vec::new();
            let mut i = 0;
            let mut j = 0;
            while i < nums1.len() || j < nums2.len() {
                if nums1[i..] > nums2[j..] {
                    result.push(nums1[i]);
                    i += 1;
                } else {
                    result.push(nums2[j]);
                    j += 1;
                }
            }
            result
        }

        let k = k as usize; 
        let m = nums1.len();
        let n = nums2.len();
        let mut result = Vec::new();

        for i in 0..=k {
            if i <= m && k - i <= n {
                let subseq1 = max_subsequence(&nums1, i);
                let subseq2 = max_subsequence(&nums2, k - i);
                let merged = merge(subseq1, subseq2);
                if merged > result {
                    result = merged;
                }
            }
        }

        result
    }
}
